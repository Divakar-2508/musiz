use std::{io::BufReader, fs::File, time::Duration};

use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct MusicPlayer {
    queue: Vec<String>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    current_song: Sink,
}

impl MusicPlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let queue = Vec::new();
        Self {
            _stream,
            stream_handle,
            queue,
            current_song: Sink::new_idle().0,
        }
    }

    pub fn add(&mut self, song: String) {
        self.queue.push(song);
    }

    pub fn play(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        let current_song = self.queue.remove(0);
        let file = BufReader::new(File::open(current_song).unwrap());
        self.current_song = self.stream_handle.play_once(file).unwrap();
        std::thread::sleep(Duration::from_secs(10));
    }

    pub fn pause(&self) {

    }

    pub fn stop(&self) {
        
    }

    pub fn remove(&mut self, index: usize) -> Option<String> {
        if self.queue.len() > index-1 {
            Some(self.queue.remove(index-1))
        } else {
            None
        }
    }
}