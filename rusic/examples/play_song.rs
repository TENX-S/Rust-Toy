
use rusic::*;
use std::io::stdin;
use std::collections::HashMap;


// ATTENTION! : - Before you run this example, you should run the example of ncm_test in decrypt_ncm before

fn main() {

    let music_list = get_music_list("decrypt_ncm/ncm_files");
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
        println!("Playing {}\nTime {:?}", music_file, get_music_time(music_file));

        _play(music_file);


    }

}
