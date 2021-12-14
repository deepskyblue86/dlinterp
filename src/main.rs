use std::io::Write;

fn main() {
    let param: Vec<String> = std::env::args().collect();

    if param.len() < 2 {
        println!("Missing argument");
        std::process::exit(1);
    }
    let param = &param[1];

    let file: elf::File;
    {
        let path = std::path::PathBuf::from(param);
        file = elf::File::open_path(&path).unwrap_or_else(|e| panic!("Error: {:?}", e));
    }

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
