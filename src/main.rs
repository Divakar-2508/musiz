pub mod music_player;

use music_player::{Song, MusicPlayer};

// fn sleep(dur: u64) {
//     let duration = Duration::from_secs(dur);
//     thread::sleep(duration);
// }

fn main() {
    let player = MusicPlayer::new();
    println!("yo1");
    player.add_song(Song::from_str("test.mp3").unwrap());
    println!("Song started to play!");
}
