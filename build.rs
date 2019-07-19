use protoc_rust::Customize;
use std::fs::{self, File};
use std::io::Write;
use std::process;

fn main() {
    // First check if the src/protos directory is empty or not...
    match fs::read_dir("./src/protos") {
        Ok(src_dir_contents) => {
            if src_dir_contents.count() > 0 {
                println!("src/protos is non-empty, not building protobufs.");
                process::exit(0);
            }
        }
        _ => {
            println!("No src/protos directory found.");
            match fs::create_dir("./src/protos") {
                Ok(_) => println!("src/protos directory created."),
                Err(e) => {
                    eprintln!("Failed to create src/protos: {}", e);
                    process::exit(1);
                }
            }
        }
    };

    let dir_contents = fs::read_dir("./protos").unwrap();
    let mut file_names = Vec::new();

    for entry in dir_contents {
        let file_name = entry.unwrap().file_name().into_string().unwrap();
        let mut full_path = "protos/".to_owned();
        full_path.push_str(file_name.as_str());
        file_names.push(full_path);
    }

    println!("Found {} protobuf files to compile.", file_names.len());

    let file_names = file_names.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    match protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &file_names[..],
        includes: &["protos"],
        customize: Customize {
            ..Default::default()
        },
    }) {
        Ok(_) => println!("Protobufs compiled successfully."),
        Err(e) => {
            eprintln!("Error compiling protobufs: {}", e);
            process::exit(1);
        }
    }

    match gen_mod_rs() {
        Ok(_) => println!("Generated src/protos/mod.rs"),
        Err(e) => {
            eprintln!("Failed to generate src/protos/mod.rs: {}", e);
            process::exit(1);
        }
    }
}

fn gen_mod_rs() -> std::io::Result<()> {
    let mut modrs = File::create("./src/protos/mod.rs")?;

    modrs.write_all(b"pub mod demo;\n")?;

    Ok(())
}
