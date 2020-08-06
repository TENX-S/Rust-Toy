
use rusic::*;
use std::{ env, io::stdin, collections::HashMap };



fn main() {

    let music_dir = env::args().skip(1).collect::<String>();

    let music_list = get_music_list(&music_dir);
    let mut play_list = HashMap::new();
    let mut idx = 1;
    for file in &music_list {
        let _song: Vec<&str> = file.to_str().unwrap().split("/").collect();
        let _song_name = _song[_song.len()-1];
        println!("{:^3}: {}", idx, _song_name);
        play_list.insert(idx, file);
        idx += 1;
    }

    loop {
        let mut num = String::new();
        stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        if num == "q\n".to_string() {
            break;
        }

        let music_file = play_list[&num.trim().parse::<i32>().unwrap()].to_str().unwrap();
        let music_time = get_music_time(music_file);
        println!("Playing {}\tTime: {:>2}m {:0>2}s", music_file, music_time.0, music_time.1);

        _play(music_file);

    }

}
