pub mod music_player;
use std::{io::Write, path::PathBuf};

use music_player::MusicPlayer;

fn main() {
    let mut player = MusicPlayer::new();
    println!("Music Player Started");
    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let args = input.split_whitespace().collect::<Vec<&str>>();
        match args[0].trim() {
            "add" => {
                let song_name = String::from(args[1].trim()) + ".mp3";
                let path = PathBuf::from(song_name);
                if !path.is_file() {
                    println!("Enter a valid song name");
                } else {
                    player.add(path);
                }
            },
            "play" => {
                if args.len() > 1 {
                    player.play(2);
                    continue;
                } 
                if let Some(value) = player.play(0) {
                    println!("{}", value);
                }
            },
            "prev" => {
                if let Some(value) = player.prev() {
                    println!("\nNow Playing: [{}] {}", value.0, value.1);
                }
            },
            "pause" => player.pause(),
            "next" => {
                if let Some(value) = player.skip_track() {
                    println!("\nNow Playing: [{}] {}",value.0, value.1);
                }
            },
            "stop" => break,
            "remove" => {
                match args[1].parse::<usize>() {
                    Ok(value) => {
                        if player.len() > value {
                            player.remove(value);
                        } else {
                            println!("Enter a valid index to remove");
                        }
                    }
                    Err(_) => {
                        println!("Enter a valid index to remove");
                    }
                }

            },
            "list" => {
                for (index, song_name) in player.get_list().into_iter().enumerate() {
                    println!("{}. {}",index+1, song_name);
                }
            }
            _ => println!("Invalid command!"),
        }
    }
    // println!("Playing!!");
}