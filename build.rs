use protoc_rust::Customize;
use std::fs;
use std::process;

fn main() {
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
}
