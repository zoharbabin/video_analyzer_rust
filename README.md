# video_analyzer_rust

Rust-based video analysis tool that leverages the power of the FFmpeg library to process video files. It extracts metadata, counts frames, identifies key frames, and measures the duration of the video. The tool is designed to be efficient and can utilize multiple CPU cores for faster processing.

## Installation

To install this project, you need to have Rust and Cargo installed on your system. You can follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust and Cargo.

1. Clone the repository:
   ```sh
   git clone https://github.com/zoharbabin/video_analyzer_rust.git
   cd video_analyzer_rust
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

## Dependencies

This project relies on the following dependencies:

- `ffmpeg-next = "6.0.0"`
- `indicatif = "0.17.6"`
- `num_cpus = "1.16.0"`
- `thiserror = "1.0.47"`

## Usage

To use this tool, run the following command:
```sh
cargo run --release -- <path_to_video_file> [<number_of_threads>]
```

- `<path_to_video_file>`: The path to the video file you want to analyze.
- `[<number_of_threads>]`: (Optional) The number of threads to use for processing. If not provided, it defaults to 1. If set to -1, it will use the number of available CPU cores.

### Example

```sh
cargo run --release -- sample_video.mp4 4
```

This command will analyze the `sample_video.mp4` file using 4 threads.

## Contribution Guidelines

We welcome contributions to this project. To contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes.
4. Test your changes thoroughly.
5. Submit a pull request with a clear description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Project Capabilities

- Extracts basic metadata from video files.
- Counts the total number of frames in the video.
- Identifies key frames in the video.
- Measures the duration of the video.
- Utilizes multiple CPU cores for faster processing.

## How to Contribute

We welcome contributions to this project. To contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes.
4. Test your changes thoroughly.
5. Submit a pull request with a clear description of your changes.

## TODO List

### Simple

- Add more detailed error handling.
- Improve the progress bar to show more information.

### Intermediate

- Add support for more video formats.
- Implement a graphical user interface (GUI).

### Advanced

- Optimize the performance for very large video files.
- Add support for distributed processing across multiple machines.
