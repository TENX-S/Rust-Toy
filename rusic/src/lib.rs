#![allow(unused)]

use glob::glob;
use rodio::Sink;
use std:: {
    env,
    fs::File,
    path::PathBuf,
    collections::HashMap,
    io::{BufReader, stdin}
};


pub fn get_music_list(_path: String) -> Vec<PathBuf> {
    let _flac = [_path.as_str(), "**", "*.flac"].iter().collect::<PathBuf>();
    let _mp3 = [_path.as_str(), "**", "*.mp3"].iter().collect::<PathBuf>();
    let _wav = [_path.as_str(), "**", "*.wav"].iter().collect::<PathBuf>();
    let _ogg = [_path.as_str(), "**", "*.ogg"].iter().collect::<PathBuf>();

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

pub fn _play(music_file: &str) {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = std::fs::File::open(music_file).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn music_list() {
        // MARK : - Remember first to run the example ncm_test in decrypt_ncm or this test will be failed

        let list = get_music_list("../decrypt_ncm/ncm_files".to_string());
        assert_eq!(list.len(), 2);
        println!("Music List: {:?}", list);
    }

}