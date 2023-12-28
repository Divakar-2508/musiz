use std::{io::BufReader, fs::File, path::PathBuf};

use rodio::{OutputStream, OutputStreamHandle, Sink};


pub struct MusicPlayer {
    queue: Vec<PathBuf>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    current_song: Sink,
    index: i32,
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
            index: 0,
        }
    }

    pub fn add(&mut self, song: PathBuf) {
        self.queue.push(song);
    }

    pub fn play(&mut self, mode: i32) -> Option<String> {
        if mode == 0 {
            if self.current_song.is_paused() {
                self.resume();
                return Some("Playing paused Song".to_string());
            }
    
            if self.queue.is_empty() {
                return  Some("Queue is Empty!" .to_string());
            }
        }
        if mode == 2 {
            
        }
        dbg!(self.get_list());
        let current_song = self.queue[self.index as usize].clone();
        if let Ok(file) = File::open(current_song) {
            let file = BufReader::new(file);
            self.current_song = self.stream_handle.play_once(file).unwrap();
        }
        return None;
    }

    pub fn resume(&self) {
        self.current_song.play();
    }

    pub fn pause(&self) {
        self.current_song.pause();
    }

    pub fn stop(&self) {
        self.current_song.stop();
    }

    pub fn remove(&mut self, index: usize){
        if self.queue.len() > index-1 {
            self.queue.remove(index-1);
        }
    }

    pub fn skip_track(&mut self) -> Option<(i32, String)> {
        self.stop();
        self.current_song.clear();
        if !self.queue.len() < (self.index + 1) as usize {
            return None;
        }

        self.index += 1;
        self.play(1);
        dbg!(self.index);
        dbg!(self.get_list());
        return Some((self.index + 1, self.queue[self.index as usize].to_string_lossy().to_string()));
    }

    pub fn prev(&mut self) -> Option<(i32, String)> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        self.play(1);
        return Some((self.index + 1, self.queue[self.index as usize].to_string_lossy().to_string()));
    }

    pub fn get_list(&self) -> Vec<String> {
        self.queue.iter().map(|x| x.as_path().to_string_lossy().to_string()).collect()
    }

    pub fn len(&self) -> usize {
        return self.queue.len();
    }
}