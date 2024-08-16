use anyhow::{Context, Result};
use rodio::{OutputStream, OutputStreamHandle, Source, Sink};
use std::io::{self, BufReader, stdin};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

struct AudioBackend {
    sink: Arc<Mutex<Sink>>,
}

impl AudioBackend {
    fn new(stream_handle: &OutputStreamHandle) -> Result<Self> {
        let sink = Sink::try_new(stream_handle)
            .context("Failed to create audio sink")?;
        
        Ok(AudioBackend {
            sink: Arc::new(Mutex::new(sink)),
        })
    }

    fn play_sound(&self, path: &str, should_loop: bool) -> Result<()> {
        let file = File::open(path)
            .context(format!("Failed to open file: {}", path))?;
        let source = rodio::Decoder::new(BufReader::new(file))
            .context("Failed to decode audio file")?;
        
        let source: Box<dyn Source<Item = i16> + Send> = if should_loop {
            Box::new(source.repeat_infinite())
        } else {
            Box::new(source)
        };

        self.sink.lock().unwrap().append(source);
        Ok(())
    }

    fn stop(&self) {
        self.sink.lock().unwrap().stop();
    }

    fn set_volume(&self, volume: f32) {
        self.sink.lock().unwrap().set_volume(volume);
    }

    fn is_empty(&self) -> bool {
        self.sink.lock().unwrap().empty()
    }
}

fn main() -> Result<()> {
    let (stream, stream_handle) = OutputStream::try_default()
        .context("Failed to create audio output stream")?;
    let audio_backend = Arc::new(AudioBackend::new(&stream_handle)?);
    let audio_backend_clone = Arc::clone(&audio_backend);

    // Keep the stream alive
    std::mem::forget(stream);

    // Start a separate thread for user input
    thread::spawn(move || {
        loop {
            println!("Enter a command (play, stop, volume, loop, quit):");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            let command = input.trim();

            match command {
                "play" => {
                    println!("Enter the path to the audio file:");
                    let mut path = String::new();
                    stdin().read_line(&mut path).expect("Failed to read line");
                    let path = path.trim();
                    if let Err(e) = audio_backend_clone.play_sound(path, false) {
                        eprintln!("Error playing sound: {}", e);
                    }
                },
                "stop" => audio_backend_clone.stop(),
                "volume" => {
                    println!("Enter new volume (0.0 - 1.0):");
                    let mut volume = String::new();
                    stdin().read_line(&mut volume).expect("Failed to read line");
                    if let Ok(v) = volume.trim().parse::<f32>() {
                        audio_backend_clone.set_volume(v);
                    } else {
                        println!("Invalid volume input");
                    }
                },
                "loop" => {
                    println!("Enter the path to the audio file to loop:");
                    let mut path = String::new();
                    stdin().read_line(&mut path).expect("Failed to read line");
                    let path = path.trim();
                    if let Err(e) = audio_backend_clone.play_sound(path, true) {
                        eprintln!("Error playing looped sound: {}", e);
                    }
                },
                "quit" => break,
                _ => println!("Unknown command"),
            }
        }
    });

    // Keep the main thread running
    loop {
        if audio_backend.is_empty() {
            thread::sleep(std::time::Duration::from_millis(100));
        } else {
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}