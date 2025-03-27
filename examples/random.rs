// dave horner 3/25
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::Rng;
use std::f32::consts::PI;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Morse code settings
    let frequency = 600.0; // Frequency of the tone in Hz
    let unit_duration = 100; // Duration of a dot in milliseconds
    let dash_duration = 3 * unit_duration;
    let dot_duration = unit_duration;
    let element_gap = unit_duration; // Gap between elements in a character
    let character_gap = 3 * unit_duration; // Gap between characters

    // Morse code for 'N': dash (---), gap, dot (.)
    let morse_n = vec![
        (dash_duration, true),  // Dash
        (element_gap, false),  // Gap between dash and dot
        (dot_duration, true),  // Dot
    ];

    // Get the default audio host and output device
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config()?.config();

    // Shared state for producing samples
    let sample_rate = config.sample_rate.0 as f32;
    let mut sample_clock = 0f32;
    let is_playing = Arc::new(AtomicBool::new(false));
    let amplitude = Arc::new(AtomicBool::new(false)); // Fade-in/out toggle
    let smooth_mode = Arc::new(AtomicBool::new(true)); // Smooth or abrupt toggle

    // Create the stream
    let stream = device.build_output_stream(
        &config,
        {
            let is_playing_clone = Arc::clone(&is_playing);
            let amplitude_clone = Arc::clone(&amplitude);
            let smooth_mode_clone = Arc::clone(&smooth_mode);
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut current_amplitude = 0.0;
                for sample in data.iter_mut() {
                    if is_playing_clone.load(Ordering::Relaxed) {
                        if smooth_mode_clone.load(Ordering::Relaxed) {
                            // Gradual fade-in
                            if amplitude_clone.load(Ordering::Relaxed) && current_amplitude < 1.0 {
                                current_amplitude += 0.001;
                            }
                            // Generate smooth sine wave
                            *sample = current_amplitude
                                * (2.0 * PI * frequency * sample_clock / sample_rate).sin();
                        } else {
                            // Abrupt sine wave
                            *sample = (2.0 * PI * frequency * sample_clock / sample_rate).sin();
                        }
                        sample_clock = (sample_clock + 1.0) % sample_rate;
                    } else {
                        if smooth_mode_clone.load(Ordering::Relaxed) {
                            // Gradual fade-out
                            if current_amplitude > 0.0 {
                                current_amplitude -= 0.001;
                            }
                            *sample = current_amplitude
                                * (2.0 * PI * frequency * sample_clock / sample_rate).sin();
                        } else {
                            // Abrupt silence
                            *sample = 0.0;
                        }
                    }
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None, // Latency option
    )?;

    // Play the Morse code continuously
    stream.play()?;

    let mut rng = rand::thread_rng();
    loop {
        // Randomly choose smooth or abrupt mode
        let smooth = rng.gen_bool(0.5);
        smooth_mode.store(smooth, Ordering::Relaxed);

        // Play in the chosen mode for a random duration
        let play_duration = rng.gen_range(3000..7000); // Random duration in milliseconds
        let start_time = std::time::Instant::now();

        while start_time.elapsed().as_millis() < play_duration as u128 {
            for &(duration, active) in &morse_n {
                amplitude.store(true, Ordering::Relaxed);
                is_playing.store(active, Ordering::Relaxed); // Set playback state
                thread::sleep(Duration::from_millis(duration as u64));
            }
            // Add a gap between repeats of the character
            amplitude.store(false, Ordering::Relaxed);
            is_playing.store(false, Ordering::Relaxed); // Silence during character gap
            thread::sleep(Duration::from_millis(character_gap as u64));
        }
    }
}

