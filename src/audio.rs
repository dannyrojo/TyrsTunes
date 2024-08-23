use rodio::{OutputStream, Sink, Decoder};
use std::sync::mpsc;
use std::thread;
use std::fs::File;
use std::io::BufReader;

pub enum AudioCommand {
    Play(String),
    Stop,
    Pause,
    Resume,
}

pub fn spawn_audio_thread() -> mpsc::Sender<AudioCommand> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        loop {
            match rx.recv().unwrap() {
                AudioCommand::Play(path) => {
                    let file_path = path.to_string();
                    let file = BufReader::new(File::open(file_path).unwrap());
                    let source = Decoder::new(file).unwrap();
                    sink.append(source);
                }
                AudioCommand::Stop => {
                    sink.stop();
                }
                AudioCommand::Pause => {
                    sink.pause();
                }
                AudioCommand::Resume => {
                    sink.play();
                }
            }
        }
    });
    tx
}