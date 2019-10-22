use std::io::Write;
use std::env;
use std::io;
use std::fs::File;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let songs =  &args[1..];
    if songs.len() == 0{
        println!("Please specify songs as Parameters.");
        return;
    }
    let mut playlist_name = String::new();
    println!("What's the name of the playlist?");

     io::stdin().read_line(&mut playlist_name).expect("Error reading input");

     playlist_name = playlist_name.trim().to_string();
     playlist_name = playlist_name.replace(".m3u", "");
     if Path::new(&format!("{}.m3u", playlist_name)).exists(){
         println!("Error, specified m3u file already exists.");
         process::exit(1);
     }
     let mut file: File = File::create(format!("{}.m3u", playlist_name)).expect(&format!("Error creating playlist file {} ", playlist_name));

     for s in songs{
         file.write_all(format!("{}\n", s).as_bytes()).expect("Error writing to file.");
     }
}
