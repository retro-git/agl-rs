#![feature(const_option, const_option_ext)]

use clap::Parser;
use std::fs;
use agl::compiler;
use std::fs::OpenOptions;
use std::io::Write;

const VERSION : &'static str = option_env!("CARGO_PKG_VERSION").unwrap();

#[derive(Parser, Debug)]
#[command(name = "agl", version = VERSION, about = "A DSL for writing GameShark codes")]
struct Cli {
    input_files: Vec<String>,

    #[arg(short, long, value_enum)]
    mode: compiler::Mode,

    //Optionally concatenate all input files into a single output file.
    #[arg(short, long, default_value_t = false)]
    concat: bool,

    //Only used if concat is set to true - specifies the output file name.
    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.output_file.is_some() && !cli.concat {
        println!("output_file is ignored because concat is not set");
    }
    
    for (i, file) in cli.input_files.iter().enumerate() {
        let code = fs::read_to_string(file).unwrap();
        let compiled = compiler::compile(code, cli.mode);
        
        //if concat and output_file is set, write to output_file
        //if concat is set but output_file is not, append to first input file but with .gs extension
        //if concat is not set, write to input file but with .gs extension
        let output_file = match cli.concat {
            true => cli.output_file.clone().unwrap_or(cli.input_files.first().unwrap().replace(".agl", ".gs")),
            false => file.replace(".agl", ".gs"),
        };

        let comment = format!("// generated by agl v{} from {}", VERSION, file);

        //in this case, we need to overwrite any old file
        if !cli.concat || (cli.concat && i == 0) {
            fs::write(&output_file, format!("{}\n{}\n", comment, compiled)).unwrap();
        }
        else {
            //if we want to write to the same file, we need to open it in append mode
            let mut file = OpenOptions::new().append(true).create(true).open(&output_file).unwrap();
            file.write_all(format!("{}\n{}", comment, compiled).as_bytes()).unwrap();
        }
    };
}

#[test]
fn test() {
    let code = fs::read_to_string("example/block.agl").unwrap();
    let compiled = compiler::compile(code, compiler::Mode::PSX);
    println!("{:?}", compiled);
    assert_eq!(compiled, "D00681c8 0005\n300681c8 0006\nD00681c8 0005\n300681c8 0005");
}