use rodio::{OutputStream, OutputStreamHandle, Decoder, source::Source};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::io::BufReader;
use std::time::Duration;
use std::fs::File;
use crate::track;

fn initialize_audiostream() -> OutputStreamHandle {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle
}

fn playback_loop
(
    stream_handle: &OutputStreamHandle, 
    receiver: Receiver<PlaybackCommand>, 
    sender: Sender<PlaybackState>
) 
{
    let mut sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let mut playlist: Vec<track::Track> = Vec::new();
    let mut current_track: Option<track::Track> = None;
    let mut is_looping = false;

    loop {
        if let Ok(command) = receiver.try_recv() {
            match command {
                PlaybackCommand::Play(file_path) => {
                    play_track(&mut sink, &file_path);
                    current_track = Some(track::Track { file_path });
                    sender.send(PlaybackState::Playing).unwrap();
                }
                PlaybackCommand::Pause => {
                    sink.pause();
                    sender.send(PlaybackState::Paused).unwrap();
                }
                PlaybackCommand::Stop => {
                    sink.stop();
                    current_track = None;
                    playlist.clear();
                    sender.send(PlaybackState::Stopped).unwrap();
                }
                PlaybackCommand::ToggleLoop => {
                    is_looping = !is_looping;
                    sender.send(PlaybackState::Looping(is_looping)).unwrap();
                }
                PlaybackCommand::Quit => {
                    break;
                }
            }
        }

        // Check if the sink is empty and we need to play the next track
        if sink.empty() {
            if let Some(next_track) = playlist.pop_front() {
                play_track(&mut sink, &next_track.file_path);
                current_track = Some(next_track);
                sender.send(PlaybackState::Playing).unwrap();
            } else if is_looping {
                if let Some(track) = &current_track {
                    play_track(&mut sink, &track.file_path);
                    sender.send(PlaybackState::Playing).unwrap();
                }
            } else {
                current_track = None;
                sender.send(PlaybackState::Stopped).unwrap();
            }
        }

        // Sleep for a short duration to avoid busy-waiting
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn play_track(sink: &mut rodio::Sink, file_path: &str) {
    sink.stop();
    let file = BufReader::new(File::open(file_path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.play();
}

enum PlaybackCommand {
    Play(String),
    Pause,
    Stop,
    ToggleLoop,
    Quit,
}

enum PlaybackState {
    Playing,
    Paused,
    Stopped,
    Looping(bool),
}

pub struct PlaybackController {
    sender: Sender<PlaybackCommand>,
    receiver: Receiver<PlaybackState>,
    playback_state: PlaybackState,
    is_looping: bool,
    current_track: Option<track::Track>,
    next_track: Option<track::Track>,
}

impl PlaybackController {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let stream_handle = initialize_audiostream();
        
        std::thread::spawn(move || {
            playback_loop(&stream_handle, receiver, sender);
        });

        Self { sender }
    }

    pub fn play(&self, file_path: String) {
        self.sender.send(PlaybackCommand::Play(file_path)).unwrap();
    }

    pub fn pause(&self) {
        self.sender.send(PlaybackCommand::Pause).unwrap();
    }

    pub fn stop(&self) {
        self.sender.send(PlaybackCommand::Stop).unwrap();
    }

    pub fn quit(&self) {
        self.sender.send(PlaybackCommand::Quit).unwrap();
    }

    pub fn toggle_loop(&self) {
        self.sender.send(PlaybackCommand::ToggleLoop).unwrap();
    }

   
}
