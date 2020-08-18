#![allow(unused)]

use glob::glob;
use rodio::{Sink, Decoder, Source};
use std:: {
    env,
    fs::File,
    path::PathBuf,
    collections::HashMap,
    io::{BufReader, stdin}
};
use std::cmp::min;


pub fn get_music_list(_path: &str) -> Vec<PathBuf> {
    let _flac = [_path, "**", "*.flac"].iter().collect::<PathBuf>();
    let _mp3 = [_path, "**", "*.mp3"].iter().collect::<PathBuf>();
    let _wav = [_path, "**", "*.wav"].iter().collect::<PathBuf>();
    let _ogg = [_path, "**", "*.ogg"].iter().collect::<PathBuf>();

    let mut music_list = vec![_flac, _mp3, _wav, _ogg]
        .iter()
        .map(|x| {
            glob(x
                .to_str()
                .unwrap())
                .unwrap()
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
        })
        .fold(vec![], |mut acc, mut x| { acc.append(&mut x); acc });

    music_list.sort();
    music_list
}

pub fn _play(music_path: &str) {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open(music_path).unwrap();
    sink.append(Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}

pub fn get_music_time(file_path: &str) -> (u64, u64){

    let music_file
        =
        Decoder::new(BufReader::new(File::open(file_path).unwrap())).unwrap();

    let total_secs = music_file.total_duration().unwrap().as_secs();
    let seconds = total_secs % 60;
    let minutes = (total_secs - seconds) / 60;

    (minutes, seconds)

}
