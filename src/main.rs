use clap::{Parser, Subcommand};
use crate::orbit::CircularOrbit;
use crate::utils::{create_data_file, create_writer, print_data};

mod constants;
pub mod orbit;
pub mod utils;

#[derive(Parser, Debug)]
#[command(about = "Small orbital mechanics toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Circular {
        #[arg(long)]
        altitude: f64,
    },
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Circular { altitude } => {
            let orbit = match CircularOrbit::new(altitude) {
                Ok(orbit) => orbit,
                Err(message) => {
                    eprintln!("error: {}", message);
                    std::process::exit(1);
                }
            };

            let writer = create_writer("data/orbit.csv").expect("failed to create writer");
            create_data_file(writer, &orbit).expect("failed to write orbit data");

            print_data(&orbit);
        }
    }
}
