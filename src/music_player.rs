use std::{fs::File, io::BufReader};

use rodio::{Sink, OutputStream, Decoder};

pub struct Song {
    source: Decoder<BufReader<File>>,
}

impl Song {
    pub fn from_file(file: File) -> Result<Self, &'static str> {
        let bufreader = BufReader::new(file);

        let source = match Decoder::new(bufreader) {
            Ok(source) => source,
            Err(_) => {
                return Err("Can't decode, try again later");
            }
        };

        Ok(Self {
            source
        })
    }

    pub fn from_str(file_path: &str) -> Result<Song, &str> {
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return Err("Can't open the file, check previliges")
        };

        Song::from_file(file)
    }

}   

pub struct MusicPlayer {
    sink: Sink
}

impl MusicPlayer {
    pub fn new() -> Self {
        //output stream to handle the audio output.
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        //sink works as an media player.
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            sink
        }
    }

    pub fn add_song(&self, song: Song) {
        self.sink.append(song.source);
        println!("appended");
        self.sink.sleep_until_end()
    }
}