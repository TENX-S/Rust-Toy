
use std:: {
    env,error,
    fs::metadata,
    path::PathBuf
};
use glob::glob;
use decrypt_ncm::convert;
use scoped_threadpool::Pool;

fn main() -> Result<(), Box<dyn error::Error>>{


    let files_path = &env::args().collect::<Vec<String>>()[1];

    let file_list = match metadata(files_path.as_str())?.is_file() {
        true => [PathBuf::from(files_path.as_str())].to_vec(),
        false => {
            let list = [files_path.as_str(), "**", "*.ncm"].iter().collect::<PathBuf>();

            glob(list.to_str()
                .unwrap())?
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
        }
    };

    let max_workers = num_cpus::get() as u32;
    let mut pool = Pool::new(max_workers);

    let total = file_list.len();
    let mut success = 0;
    let mut failed = 0;

    pool.scoped( |scoped| {
        for file in file_list {
            scoped.execute( move || {
                match convert(file) {
                    Ok(_) => { success += 1; }
                    Err(_) => { failed += 1; }
                }
            })
        }
    });

    println!("Total: {}\nSuccess: {}\nFailed: {}", total, success, failed);
    Ok(())
}