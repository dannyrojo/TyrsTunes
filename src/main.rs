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

    fn play_or_resume(&self, track_name: &str) -> Result<()> {
        if self.sink.lock().unwrap().is_paused() {
            self.sink.lock().unwrap().play();
            println!("Resumed playback on {}", track_name);
        } else if self.sink.lock().unwrap().empty() {
            self.play_next().context(format!("Failed to play next track on {}", track_name))?;
            println!("Started playback on {}", track_name);
        }
        Ok(())
    }

    fn stop(&self, track_name: &str) {
        self.sink.lock().unwrap().pause();
        println!("Stopped playback on {}", track_name);
    }

    fn set_volume(&self, volume: f32, track_name: &str) {
        self.sink.lock().unwrap().set_volume(volume);
        println!("Set volume to {} on {}", volume, track_name);
    }

    fn is_empty(&self) -> bool {
        self.sink.lock().unwrap().empty()
    }

    fn toggle_loop(&self, track_name: &str) -> Result<()> {
        let mut is_looping = self.is_looping.lock().unwrap();
        *is_looping = !*is_looping;
        
        if *is_looping {
            println!("Looping enabled for the current track on {}", track_name);
        } else {
            println!("Looping disabled for the current track on {}", track_name);
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

    fn skip(&self, track_name: &str) -> Result<()> {
        self.sink.lock().unwrap().stop();
        self.play_next().context(format!("Failed to skip to next track on {}", track_name))?;
        println!("Skipped to next track on {}", track_name);
        Ok(())
    }
}

struct DualAudioBackend {
    playlist1: Arc<AudioBackend>,
    playlist2: Arc<AudioBackend>,
}

impl DualAudioBackend {
    fn new(stream_handle: OutputStreamHandle) -> Result<Self> {
        Ok(DualAudioBackend {
            playlist1: Arc::new(AudioBackend::new(stream_handle.clone())?),
            playlist2: Arc::new(AudioBackend::new(stream_handle)?),
        })
    }
}

fn main() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .context("Failed to create audio output stream")?;
    let dual_audio_backend = Arc::new(DualAudioBackend::new(stream_handle)?);
    let dual_audio_backend_clone = Arc::clone(&dual_audio_backend);

    let _input_thread = thread::spawn(move || {
        loop {
            println!("Enter a command (playlist1/playlist2 play/stop/volume/loop/skip/add, quit):");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            let command = input.trim();

            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() < 2 {
                println!("Invalid command format. Use 'playlist1' or 'playlist2' followed by the command.");
                continue;
            }

            let (playlist, playlist_name) = match parts[0] {
                "playlist1" => (&dual_audio_backend_clone.playlist1, "playlist1"),
                "playlist2" => (&dual_audio_backend_clone.playlist2, "playlist2"),
                _ => {
                    println!("Invalid playlist selection. Use 'playlist1' or 'playlist2'.");
                    continue;
                }
            };

            match parts[1] {
                "play" => {
                    if let Err(e) = playlist.play_or_resume(playlist_name) {
                        eprintln!("Error playing or resuming {}: {}", playlist_name, e);
                    }
                },
                "stop" => playlist.stop(playlist_name),
                "volume" => {
                    println!("Enter new volume (0.0 - 1.0):");
                    let mut volume = String::new();
                    stdin().read_line(&mut volume).expect("Failed to read line");
                    match volume.trim().parse::<f32>() {
                        Ok(v) if v >= 0.0 && v <= 1.0 => playlist.set_volume(v, playlist_name),
                        _ => println!("Invalid volume input. Please enter a number between 0.0 and 1.0."),
                    }
                },
                "loop" => {
                    if let Err(e) = playlist.toggle_loop(playlist_name) {
                        eprintln!("Error toggling loop on {}: {}", playlist_name, e);
                    }
                },
                "skip" => {
                    let playlist_clone = Arc::clone(playlist);
                    let playlist_name = playlist_name.to_string();
                    thread::spawn(move || {
                        if let Err(e) = playlist_clone.skip(&playlist_name) {
                            eprintln!("Error skipping track on {}: {}", playlist_name, e);
                        }
                    });
                },
                "add" => {
                    println!("Enter the path to the audio file:");
                    let mut path = String::new();
                    stdin().read_line(&mut path).expect("Failed to read line");
                    let path = path.trim().to_string();
                    if let Err(e) = playlist.add_to_playlist(path) {
                        eprintln!("Error adding to {}: {}", playlist_name, e);
                    } else {
                        println!("Added track to {}", playlist_name);
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
        if dual_audio_backend.playlist1.is_empty() {
            if let Err(e) = dual_audio_backend.playlist1.handle_track_end() {
                eprintln!("Error handling playlist1 end: {}", e);
            }
        }
        if dual_audio_backend.playlist2.is_empty() {
            if let Err(e) = dual_audio_backend.playlist2.handle_track_end() {
                eprintln!("Error handling playlist2 end: {}", e);
            }
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
}