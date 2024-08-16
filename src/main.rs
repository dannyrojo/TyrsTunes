use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn main() -> Result<(), Box<dyn Error>> {
    // The path to the audio file
    let path = Path::new("/home/eggbert/songs/Divine1.mp3");

    // Initialize the Rodio output stream (default device)
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Create an idle sink (not automatically connected to a stream)
    let (sink, queue) = Sink::new_idle();

    // Open the audio file
    let file = File::open(path)?;

    // Use a buffered reader to read the file and decode it
    let source = Decoder::new(BufReader::new(file))?;

    // Append the audio source to the sink
    sink.append(source);

    // Manually play the sink using the stream handle
    stream_handle.play_raw(queue)?;

    // Keep the program alive while the audio is playing
    sink.sleep_until_end();

    Ok(())
}
