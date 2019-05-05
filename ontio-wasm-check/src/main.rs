extern crate clap;
use clap::{App,Arg};
use invalid_check::check_wasm::check;
fn main() {
    let matches = App::new("Check")
        .version("0.1.0")
        .author("lucas")
        .about("check wasm valid")
        .arg(Arg::with_name("WASM_DIR")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("wasm dir to check"))
        .get_matches();
    let wasm_dir = matches.value_of("WASM_DIR").unwrap();
    if wasm_dir == "" {
        println!("file name is None");
    }
    if !wasm_dir.ends_with(".wasm") {
        println!("file extension is wrong")
    }
    check(wasm_dir);
}