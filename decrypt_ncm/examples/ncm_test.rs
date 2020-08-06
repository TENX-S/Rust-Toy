
use std::env;
use decrypt_ncm::decrypt_ncm;


fn main() {
    let ncm_path = env::args().skip(1).collect::<String>();

    decrypt_ncm(&ncm_path).unwrap();
}