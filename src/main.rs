use std::io::Write;

fn main() {
    let filename = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Missing argument");
            std::process::exit(1)
        }
    };

    let file = elf::File::open_path(filename).unwrap();

    let data = file
        .get_section(".interp")
        .expect("Failed to look up .interp section")
        .data
        .as_slice();

    // skip the trailing \0
    std::io::stdout()
        .write_all(&data[..data.len() - 1])
        .unwrap();
    println!();
}
