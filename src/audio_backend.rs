use anyhow::{Context, Result};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::io::BufReader;
use std::fs::File;
use std::sync::{Arc, Mutex};
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
    fn new(stream_handle: &OutputStreamHandle) -> Result<Self> {
        let sink = Sink::try_new(stream_handle)
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

struct DualAudioBackend {
    track1: Arc<AudioBackend>,
    track2: Arc<AudioBackend>,
}

impl DualAudioBackend {
    fn new(stream_handle: &OutputStreamHandle) -> Result<Self> {
        Ok(DualAudioBackend {
            track1: Arc::new(AudioBackend::new(stream_handle)?),
            track2: Arc::new(AudioBackend::new(stream_handle)?),
        })
    }
}

pub struct AudioPlayer {
    dual_audio_backend: Arc<DualAudioBackend>,
    _stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()
            .context("Failed to create audio output stream")?;
        let dual_audio_backend = Arc::new(DualAudioBackend::new(&stream_handle)?);

        Ok(AudioPlayer {
            dual_audio_backend,
            _stream: stream,
        })
    }

    pub fn play_or_resume(&self, playlist: usize) -> Result<()> {
        let backend = self.get_backend(playlist);
        backend.play_or_resume()
    }

    pub fn stop(&self, playlist: usize) {
        let backend = self.get_backend(playlist);
        backend.stop();
    }

    pub fn set_volume(&self, playlist: usize, volume: f32) {
        let backend = self.get_backend(playlist);
        backend.set_volume(volume);
    }

    pub fn toggle_loop(&self, playlist: usize) -> Result<()> {
        let backend = self.get_backend(playlist);
        backend.toggle_loop()
    }

    pub fn skip(&self, playlist: usize) -> Result<()> {
        let backend = self.get_backend(playlist);
        backend.skip()
    }

    pub fn add_to_playlist(&self, playlist: usize, path: String) -> Result<()> {
        let backend = self.get_backend(playlist);
        backend.add_to_playlist(path)
    }

    pub fn get_volume(&self, playlist: usize) -> f32 {
        let backend = self.get_backend(playlist);
        backend.sink.lock().unwrap().volume()
    }

    fn get_backend(&self, playlist: usize) -> &AudioBackend {
        match playlist {
            1 => &self.dual_audio_backend.track1,
            2 => &self.dual_audio_backend.track2,
            _ => panic!("Invalid playlist number"),
        }
    }

    pub fn update(&self) -> Result<()> {
        if self.dual_audio_backend.track1.is_empty() {
            self.dual_audio_backend.track1.handle_track_end()?;
        }
        if self.dual_audio_backend.track2.is_empty() {
            self.dual_audio_backend.track2.handle_track_end()?;
        }
        Ok(())
    }
}