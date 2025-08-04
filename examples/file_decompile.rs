use luadec::decompile_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file.luac>", args[0]);
        eprintln!("Decompiles a Lua bytecode file and prints the result");
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    println!("Decompiling: {}", filename);
    
    match decompile_file(filename) {
        Ok(source) => {
            println!("--- Decompiled Source ---");
            println!("{}", source);
        }
        Err(e) => {
            eprintln!("Error decompiling {}: {}", filename, e);
            std::process::exit(1);
        }
    }
}