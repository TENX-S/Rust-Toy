#![allow(unused)]

extern crate dirs;
extern crate sys_info;

use grp::*;
use chrono::prelude::*;
use std::{
    io::prelude::*,
    env, path::Path,
    fs::{ File, OpenOptions }
};



fn main()
{

    let requirement = env::args()
                                      .skip(1)
                                      .map(|arg| arg.parse::<BigUint>().expect("Should be positive"))
                                      .collect::<Vec<_>>(); // get the user's input and parse them into numbers which should be positive

    if !requirement.is_empty()
    {

        let (length,          sbl_cnt,         num_cnt)
            =
            (requirement[0].clone(), requirement[1].clone(), requirement[2].clone());

        if length >= sbl_cnt.clone() + num_cnt.clone()
        {
            save_to_desktop(&RandomPassword::new(length, sbl_cnt, num_cnt).show());
        }

        else
        {
            println!("Invalid input!!");
        }

    }

    else // Default
    {
        println!("{}", RandomPassword::new(12, 1, 3).show());
    }

}


fn save_to_desktop(rp: &str) -> std::io::Result<()>
{

    let home = dirs::desktop_dir().unwrap();
    let mut filepath = String::new();

    match sys_info::os_type().unwrap().as_str()
    {
        "Darwin" | "Linux" => { filepath = format!("{}/random_password.txt", home.to_str().unwrap()); },

        "Windows" => { filepath = format!("{}\\random_password.txt", home.to_str().unwrap()); },

        _ => ()
    }

    // if cfg!(target_os = "macos") {
    //     filepath = format!("{}/random_password.txt", home.to_str().unwrap());
    // } else if cfg!(target_os = "windows") {
    //     filepath = format!("{}\\random_password.txt", home.to_str().unwrap());
    // }

    let mut file: File;

    if !Path::new(filepath.as_str()).exists()
        &&
        Path::new(filepath.as_str()).is_file()
    {
        file = File::create(filepath.as_str())?;
    }

    file = OpenOptions::new()
                       .append(true)
                       .open(filepath.as_str())?;

    writeln!(&mut file, "{}\n{}\n", now_time(), rp)?;

    Ok(())

}


#[inline]
fn now_time() -> String {

    Local::now()
          .time()
          .to_string()
          .chars()
          .into_iter()
          .map(|c| c.to_string())
          .collect::<Vec<_>>()[..8]
          .join("")

}
