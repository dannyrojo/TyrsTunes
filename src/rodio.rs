use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn play_audio(file_path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file_path);

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let (sink, queue) = Sink::new_idle();

    let file = File::open(path)?;
    let source = Decoder::new(BufReader::new(file))?;

    sink.append(source);
    stream_handle.play_raw(queue)?;

    sink.sleep_until_end();

    Ok(())
}
