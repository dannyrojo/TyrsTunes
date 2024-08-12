use anyhow::{Context, Result};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::io::{BufReader, stdin};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Track {
    path: String,
}

struct AudioBackend {
    sink: Arc<Mutex<Sink>>,
    playlist: Arc<Mutex<VecDeque<Track>>>,
    current_track_index: Arc<Mutex<Option<usize>>>,
    is_looping: Arc<Mutex<bool>>,
}

impl AudioBackend {
    fn new(stream_handle: OutputStreamHandle) -> Result<Self> {
        let sink = Sink::try_new(&stream_handle)
            .context("Failed to create audio sink")?;
        
        Ok(AudioBackend {
            sink: Arc::new(Mutex::new(sink)),
            playlist: Arc::new(Mutex::new(VecDeque::new())),
            current_track_index: Arc::new(Mutex::new(None)),
            is_looping: Arc::new(Mutex::new(false)),
        })
    }

    fn add_to_playlist(&self, path: String) -> Result<()> {
        self.playlist.lock().unwrap().push_back(Track { path });
        self.play_next_if_empty()?;
        Ok(())
    }

    fn play_next_if_empty(&self) -> Result<()> {
        if self.sink.lock().unwrap().empty() {
            self.play_next()?;
        }
        Ok(())
    }

    fn play_next(&self) -> Result<()> {
        let playlist = self.playlist.lock().unwrap();
        let mut current_track_index = self.current_track_index.lock().unwrap();
        
        if playlist.is_empty() {
            *current_track_index = None;
            return Ok(());
        }

        let next_index = current_track_index
            .map(|i| (i + 1) % playlist.len())
            .unwrap_or(0);

        let track = playlist[next_index].clone();
        self.play_track(track)?;
        *current_track_index = Some(next_index);
        
        Ok(())
    }

    fn play_track(&self, track: Track) -> Result<()> {
        let file = File::open(&track.path)
            .context(format!("Failed to open file: {}", track.path))?;
        let source = rodio::Decoder::new(BufReader::new(file))
            .context("Failed to decode audio file")?;
        
        let sink = self.sink.lock().unwrap();
        sink.append(source);
        
        
        Ok(())
    }

    fn play_or_resume(&self) -> Result<()> {
        if self.sink.lock().unwrap().is_paused() {
            self.sink.lock().unwrap().play();
        } else if self.sink.lock().unwrap().empty() {
            self.play_next()?;
        }
        Ok(())
    }

    fn stop(&self) {
        self.sink.lock().unwrap().pause();
    }

    fn set_volume(&self, volume: f32) {
        self.sink.lock().unwrap().set_volume(volume);
    }

    fn is_empty(&self) -> bool {
        self.sink.lock().unwrap().empty()
    }

    fn toggle_loop(&self) -> Result<()> {
        let mut is_looping = self.is_looping.lock().unwrap();
        *is_looping = !*is_looping;
        
        if *is_looping {
            println!("Looping enabled for the current track.");
        } else {
            println!("Looping disabled for the current track.");
        }
        Ok(())
    }

    fn handle_track_end(&self) -> Result<()> {
        if *self.is_looping.lock().unwrap() {
            if let Some(index) = *self.current_track_index.lock().unwrap() {
                let track = self.playlist.lock().unwrap()[index].clone();
                self.play_track(track)?;
            }
        } else {
            self.play_next()?;
        }
        Ok(())
    }

    fn skip(&self) -> Result<()> {
        self.sink.lock().unwrap().stop();
        self.play_next()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .context("Failed to create audio output stream")?;
    let audio_backend = Arc::new(AudioBackend::new(stream_handle)?);
    let audio_backend_clone = Arc::clone(&audio_backend);

    let _input_thread = thread::spawn(move || {
        loop {
            println!("Enter a command (play, stop, volume, loop, skip, add, quit):");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            let command = input.trim();

            match command {
                "play" => {
                    if let Err(e) = audio_backend_clone.play_or_resume() {
                        eprintln!("Error playing or resuming track: {}", e);
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
                    if let Err(e) = audio_backend_clone.toggle_loop() {
                        eprintln!("Error toggling loop: {}", e);
                    }
                },
                "skip" => {
                    let audio_backend = Arc::clone(&audio_backend_clone);
                    thread::spawn(move || {
                        if let Err(e) = audio_backend.skip() {
                            eprintln!("Error skipping track: {}", e);
                        }
                    });
                },
                "add" => {
                    println!("Enter the path to the audio file:");
                    let mut path = String::new();
                    stdin().read_line(&mut path).expect("Failed to read line");
                    let path = path.trim().to_string();
                    if let Err(e) = audio_backend_clone.add_to_playlist(path) {
                        eprintln!("Error adding to playlist: {}", e);
                    }
                },
                "quit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                },
                _ => println!("Unknown command"),
            }
        }
    });

    loop {
        if audio_backend.is_empty() {
            if let Err(e) = audio_backend.handle_track_end() {
                eprintln!("Error handling track end: {}", e);
            }
            thread::sleep(std::time::Duration::from_millis(100));
        } else {
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}