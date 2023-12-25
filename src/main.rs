pub mod music_player;
use music_player::MusicPlayer;

fn main() {
    let mut player = MusicPlayer::new();
    player.add("test.mp3".to_string());
    player.add("test2.mp3".to_string());
    player.remove(1);
    player.play();
    println!("Playing!!");
}
