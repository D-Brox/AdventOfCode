use seq_macro::seq;
seq!( N in 01..=10 {

use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::{Parser, Subcommand, ValueEnum};

mod days {
    #(pub mod day~N;)*
}
use days::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum Part {
    Part1,
    Part2,
}

#[derive(Subcommand)]
enum Commands {#(
    Day~N{
        #[arg(value_enum)]
        part: Part,
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
    },
)*}

fn main() {
    let cli = Cli::parse();
        match &cli.command {#(
            Commands::Day~N{part,input} => {
                let input:Vec<String> = read_lines(input)
                    .unwrap()
                    .map(|l|l.unwrap())
                    .collect();
                let now = Instant::now();
                match part{
                    Part::Part1 => print!("{:?} ",day~N::part1(input)),
                    Part::Part2 => print!("{:?} ",day~N::part2(input)),
                }
                println!("(in {:.2?})", now.elapsed());
            },
        )*}
}

});
