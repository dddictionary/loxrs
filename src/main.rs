use std::{fs, env, io, process};
use std::error::Error;

fn run(file_path: &String) -> Result<(), io::Error>{
    todo!("file_path = {}", file_path);
}

fn repl() {
    todo!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: loxrs <script>")
    } else if args.len() == 1 {
        run(&args[0]).unwrap_or_else(|e| {
            eprintln!("Problem running: {e}");
            process::exit(1)
        });
    } else {
        repl();
    }

}
