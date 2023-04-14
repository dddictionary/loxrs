use std::{fs, env, io, process};

fn run(file_path: &String) -> Result<(), io::Error>{
    Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
}

fn repl() {
    todo!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}", args);
    if args.len() > 2 {
        println!("Usage: loxrs <script>")
    } else if args.len() == 2 {
        run(&args[0]).unwrap_or_else(|e| {
            eprintln!("Problem running: {e}");
            process::exit(1)
        });
    } else {
        repl();
    }

}
