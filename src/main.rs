use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[clap(about = "CLI for pixe.la")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Increment the specified graph pixel")]
    Inc {
        #[clap(help = "Graph ID to increment pixel", required = true)]
        graph: String,
    },
    #[clap(about = "Get the specified resource(s)")]
    Get {
        #[clap(help = "Name of the resource(s)", required = true)]
        resource: String,
    },
}

fn main() {
    let args = Cli::parse();

    let mut pb = dirs::home_dir().unwrap();
    pb.push(".config");
    pb.push("pixela");
    let path = pb.to_str().unwrap().to_string();

    let mut file = File::open(path).expect("configuration file not find");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("fail to read configuration");
    println!("content={}", contents);

    match &args.command {
        Commands::Inc { graph } => {
            todo!("increment {} pixel", graph);
        }
        Commands::Get { resource } => {
            todo!("get {} resource", resource);
        }
    };
}
