use std::fs::File;
use std::{io, env};
use fs2::FileExt;
use std::process::exit;
use std::io::{Read};
use std::path::Path;

fn main() {
    let args :Vec<String>= env::args().collect();

    let mut use_shared_lock = false;

    if args.len() < 2 || args.len() > 3 || &args[1] == "-h" || &args[1] == "--help" || &args[1] == "/h" || &args[1] == "/help" || &args[1] == "/?" {
        print_usage();
    }

    if &args[1]  == "-v" || &args[1] == "--version" {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let mut file_path = Path::new(&args[1]);

    if args.len() == 3 && ( &args[1] == "-s" || &args[1] == "--shared" ) {
        use_shared_lock = true;
        file_path = Path::new(&args[2]);
    }

    if !file_path.exists() {
        println!("Target file does not exist.");
        print_usage();
        exit(2);
    }

    println!("Preparing to lock the file.");
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file: {}", err);
            exit(3);
        }
    };
    let file_lock = if use_shared_lock { file.lock_shared() } else { file.lock_exclusive() };
    if let Err(err) = file_lock {
        println!("Failed to obtain exclusive file lock: {}", err);
        exit(4);
    };


    let lock_type_text = match use_shared_lock {
        true => "SHARED",
        false => "EXCLUSIVE"
    };

    println!("Obtained {} file lock.", lock_type_text);
    println!("Press ENTER to release file lock. ");

    io::stdin()
        .read(&mut [0]).unwrap();

    if let Err(err) = file.unlock() {
        println!("Failed to release exclusive file lock, sorry for that.\n{}", err);
        exit(5);
    };
    println!("Released file lock.");
}

fn print_usage() {
    let exe_name = &env::args().collect::<Vec<String>>()[0];
    println!("\nfilelock v{} by Stefan Marx", env!("CARGO_PKG_VERSION"));
    println!("\nUsage: {} [OPTIONS] file_path", exe_name);
    println!("\nOptions:");
    println!(" -s\t--shared\tLock the file shared and not exclusively\n");
    println!(" -v\t--version\tDisplay version information\n");
    exit(1);
}