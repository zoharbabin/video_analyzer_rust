extern crate ffmpeg_next as ffmpeg;
extern crate num_cpus;

use std::time::Instant;
use ffmpeg::format::input;
use ffmpeg::media::Type;
use ffmpeg::codec::threading;
use indicatif::{ProgressBar, ProgressStyle};
use thiserror::Error;

// Define an enumeration for custom error types.
#[derive(Error, Debug)]
enum MyError {
    #[error("No video stream found")]
    NoVideoStreamError,
    
    #[error(transparent)]
    FfmpegError(#[from] ffmpeg::Error),
}

// Implement the `From` trait to allow converting a `MyError` into an `ffmpeg::Error`.
impl From<MyError> for ffmpeg::Error {
    fn from(err: MyError) -> Self {
        match err {
            MyError::NoVideoStreamError => ffmpeg::Error::Unknown,
            MyError::FfmpegError(e) => e,
        }
    }
}

fn millis_to_clock_format(ms: u64) -> String {
  let hours = ms / 3_600_000;
  let mins = (ms % 3_600_000) / 60_000;
  let secs = (ms % 60_000) / 1000;
  let millis = ms % 1000;

  if hours > 0 {
      format!("{:02}h {:02}m {:02}s .{:03}ms", hours, mins, secs, millis)
  } else {
      format!("{:02}m {:02}s .{:03}ms", mins, secs, millis)
  }
}

fn format_with_commas(num: i64) -> String {
  let num_str = num.to_string();
  let chars: Vec<_> = num_str.chars().rev().enumerate().map(|(i, c)| {
      if i != 0 && i % 3 == 0 {
          vec![',', c]
      } else {
          vec![c]
      }
  }).flatten().collect();
  chars.into_iter().rev().collect()
}

// Entry point of the program.
fn main() -> Result<(), ffmpeg::Error> {
  // Initialize the ffmpeg library.
  ffmpeg::init().unwrap();

  // Capture the current time to measure duration later.
  let start = Instant::now();

  // Extract the file path from the command-line arguments. Return an error if no path is provided.
  let path = std::env::args()
    .nth(1)
    .ok_or(MyError::NoVideoStreamError)?;

  // Extract thread count from command-line arguments. If not provided, it defaults to 1.
  let threads_number_input: i32 = std::env::args()
    .nth(2)
    .map_or(1, |s| s.parse::<i32>().unwrap_or(1));
  let mut threads_number: usize = 1;
  // Check and warn if invalid thread count is provided.
  if threads_number_input == -1 {
    threads_number = num_cpus::get();
    println!("Setting threading to the number of available cores: {}.", threads_number);
  } else { 
    if threads_number_input < 1 {
      println!("invalid thread count provided. Defaulting to 1 thread.");
    }
  }
  
  // Open the input video file.
  let mut ictx = input(&path)?;
  
  // Fetch the best video stream from the file. If there's none, return a StreamNotFound error.
  let input = ictx
    .streams()
    .best(Type::Video)
    .ok_or(ffmpeg::Error::StreamNotFound)?;
  
  // Get the index of the video stream.
  let video_stream_index = input.index();

  // Create a decoding context for the video stream.
  let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
  let mut decoder = context_decoder.decoder().video()?;
  // Create a threading config and set the number of threads.
  let threading_config = threading::Config { 
    kind: threading::Type::Frame, 
    count: threads_number 
  };
  decoder.set_threading(threading_config);

  let time_base = input.time_base();

  // Initialize counters to track the last key frame and total frame count.
  let mut last_frame = 0;
  let mut frame_count = 0;
  
  // Initialize highest_pts as an Option<i64> with None to indicate it hasn't been set yet.
  let mut highest_dts: Option<i64> = None;

  // Create an empty video frame outside the loop to reuse it.
  let mut frame = ffmpeg::util::frame::video::Video::empty();

  let pb = ProgressBar::new_spinner();
  let style = ProgressStyle::default_spinner()
    .tick_chars("/|\\- ")
    .template("{spinner:.green} {msg}");
  pb.set_style(style.unwrap());
  pb.set_message("Processing packets...");
  
  // Iterate over each packet in the video stream.
  for (stream, packet) in ictx.packets() {
    pb.tick();
    // Skip packets that aren't from the video stream.
    if stream.index() != video_stream_index {
      continue;
    }

    // Compare and update the highest_dts.
    highest_dts = match highest_dts {
      Some(val) if packet.dts().is_some() => {
          if packet.dts().unwrap() > val {
              packet.dts()
          } else {
              highest_dts
          }
      }
      None => packet.dts(),
      _ => highest_dts,
    };
    
    // Send the packet to the decoder.
    decoder.send_packet(&packet)?;
    
    // Fetch decoded frames from the decoder. Loop until no more frames are available.
    while decoder.receive_frame(&mut frame).is_ok() {
      // If the frame is a key frame, update the last_frame counter.
      if frame.is_key() {
          last_frame = frame_count;
      }
      // Increment the frame count.
      frame_count += 1;
    }
  }
  pb.finish_with_message("Processing complete.");

  // Calculate the time taken to process the video.
  let code_execution_time = start.elapsed();
  let code_execution_time_ms = code_execution_time.as_millis() as u64;
  
  // Get the media duration. You might need to change `numerator()` and `denominator()`
  // to the correct methods or fields if they are named differently.
  let media_duration_ms = (ictx.duration() as f64 * time_base.denominator() as f64 / time_base.numerator() as f64 * 1000.0) as u64;

  // Get the media duration based on last frame.
  let mut last_frame_ms = 0;
  if let Some(h_dts) = highest_dts {
    last_frame_ms = (h_dts as f64 * time_base.numerator() as f64 / time_base.denominator() as f64 * 1000.0) as u64;
  }

  // Blue color and bold for the headers
  const BLUE_BOLD: &str = "\x1B[1;34m";
  // Reset color and style back to default
  const RESET: &str = "\x1B[0m";
  
  println!("{}Basic file metadata - {}", BLUE_BOLD, RESET);
  println!("ictx.duration: {}", ictx.duration());
  println!("Media Duration: {}", millis_to_clock_format(media_duration_ms));
  println!("Time base numerator: {}", time_base.numerator());
  println!("Time base denominator: {}", time_base.denominator());

  println!("{}{}Calculated from the frames - {}", RESET, BLUE_BOLD, RESET);
  println!("Last key frame id: {}", format_with_commas(last_frame));
  println!("Frames count: {}", format_with_commas(frame_count));
  println!("Last Frame Time: {}", millis_to_clock_format(last_frame_ms));
  println!("Code execution time: {}", millis_to_clock_format(code_execution_time_ms));

  // Return an Ok result.
  Ok(())
}
