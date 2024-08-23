use rodio::{OutputStream, OutputStreamHandle, Decoder, source::Source};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::io::BufReader;
use std::time::Duration;
use std::fs::File;
use crate::track;

pub fn initialize_audiostream() -> (OutputStream, OutputStreamHandle) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    (stream, stream_handle)
}