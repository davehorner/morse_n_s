
# Morse Code Audio Generator in Rust

This project demonstrates how to generate Morse code audio signals using Rust and the [CPAL](https://github.com/RustAudio/cpal) crate for cross-platform audio output. The example generates the Morse code for the letter **"N"** (dash-dot) at a frequency of 600 Hz, and it plays the pattern continuously with smooth fade-in and fade-out transitions.

## Features

- **Morse Code Generation:** Plays the Morse code for the letter "N" (dash followed by dot).
- **Audio Output:** Uses the CPAL crate to interface with the default audio output device.
- **Timing Control:** Implements precise timing for dots, dashes, and gaps.
- **Fade Effects:** Includes gradual fade-in and fade-out effects to smooth the transitions between tones and silence.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (Rust’s package manager)
- CPAL crate (dependency specified in `Cargo.toml`)

## Installation

1. **Clone the Repository**

   ```bash
   git clone <repository_url>
   cd <repository_directory>
   ```

2. **Build the Project**

   Build the project in release mode for optimized performance:

   ```bash
   cargo build --release
   ```

## Usage

Run the executable to start playing the Morse code for the letter "N":

```bash
cargo run --release
```

The program continuously plays the Morse code sequence until you stop it (e.g., by pressing `Ctrl+C`).

## Default Binary and Default Example

This repository includes two implementations of the Morse Code Audio Generator:

- **Default Binary (`src/main.rs`)**:  
  This is the original implementation, which continuously plays the Morse code for the letter "N" with smooth fade-in and fade-out effects. It serves as the primary executable when you run the project with `cargo run`.

- **Default Example (`examples/enhanced.rs`)**:  
  This version introduces additional functionality by using the `rand` crate to randomly toggle between smooth (gradual fade) and abrupt transitions. It also varies the playback duration for each mode, showcasing how you can expand upon the basic implementation.

## Code Overview

- **Morse Code Timing:**  
  - **Dot Duration:** 100 milliseconds  
  - **Dash Duration:** 300 milliseconds (3 times the dot duration)  
  - **Element Gap:** 100 milliseconds (gap between elements of the character)  
  - **Character Gap:** 300 milliseconds (gap between repeated characters)

- **Tone Generation:**  
  The sine wave is generated at 600 Hz with a gradually increasing amplitude (fade-in) when the tone starts, and a fade-out when it stops.

- **Audio Stream:**  
  The audio stream is created using CPAL, which accesses the system’s default audio output device. The stream continuously processes audio samples by calculating the sine wave for active tones and producing silence otherwise.

## Historical Context

The Morse code for "N" was selected for its simplicity and ease of recognition in noisy environments. This made it a practical choice for communication in emergency situations, as exemplified by its use in historical distress signals.

## Credits

- **Dave Horner** – Original implementation (3/25)
- [CPAL](https://github.com/RustAudio/cpal) – Cross-platform audio I/O library for Rust

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
