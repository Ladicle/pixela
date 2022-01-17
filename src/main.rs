use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
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
    let mut token = String::new();
    file.read_to_string(&mut token)
        .expect("fail to read configuration");
    token = token.trim().to_string();

    let client = Client::new();
    match &args.command {
        Commands::Inc { graph } => {
            let url = format!(
                "https://pixe.la/v1/users/ladicle/graphs/{}/increment",
                graph
            )
            .to_string();

            let resp = match client
                .put(url)
                .header("X-USER-TOKEN", token)
                .header(CONTENT_LENGTH, 0)
                .send()
            {
                Ok(resp) => resp,
                Err(err) => panic!("fail to send request: {}", err),
            };

            if resp.status().is_success() {
                println!("{} is incremented!", graph);
            } else {
                panic!("{:?} Error", resp.status());
            }
        }
        Commands::Get { resource } => {
            todo!("get {} resource", resource);
        }
    };
}
