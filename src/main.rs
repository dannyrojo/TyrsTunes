use anyhow::{Context, Result};
use rodio::{OutputStream, Sink};
use std::io::BufReader;
use std::fs::File;

struct AudioBackend {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioBackend {
    fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()
            .context("Failed to create audio output stream")?;
        let sink = Sink::try_new(&stream_handle)
            .context("Failed to create audio sink")?;
        
        Ok(AudioBackend {
            _stream: stream,
            sink,
        })
    }

    fn play_sound(&self, path: &str) -> Result<()> {
        let file = File::open(path)
            .context(format!("Failed to open file: {}", path))?;
        let source = rodio::Decoder::new(BufReader::new(file))
            .context("Failed to decode audio file")?;
        
        self.sink.append(source);
        Ok(())
    }

    fn stop(&self) {
        self.sink.stop();
    }

    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }
}

fn main() -> Result<()> {
    let audio_backend = AudioBackend::new()?;
    
    // Example usage
    audio_backend.play_sound("/home/eggbert/Downloads/Divine1.mp3")?;
    audio_backend.set_volume(0.5);
    
    // Keep the program running to play the sound
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    audio_backend.stop();
    Ok(())
}