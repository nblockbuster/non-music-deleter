use std::{path::Path, env, io::{BufReader, prelude::*}, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Folder argument not found.\nUsage: {} [path to folder of wavs]",args[0]);
        return
    }
    
    let folder = args[1].to_string();
    let file = File::open("OSTs.db").unwrap();
    let mut reader = BufReader::new(file);
    let mut line:String = String::new();
    reader.read_to_string(&mut line).unwrap();
    let thread = std::thread::spawn(move || {        
        for entry in std::fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let path:String = entry.path().display().to_string();
            let stem:String = Path::new(& path).file_stem().unwrap().to_str().unwrap().to_string();
            if !line.contains(&stem.as_str()) && stem.len() == 8 {
                println!("Removed {}.", stem);
                std::fs::remove_file(path).unwrap();
            }
        }
    });
    thread.join().unwrap();
}
