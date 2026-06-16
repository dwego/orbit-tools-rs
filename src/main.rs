use clap::{Parser, Subcommand};
use crate::orbit::CircularOrbit;

mod constants;
pub mod orbit;

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

            println!("Orbit type: circular");
            println!("Altitude: {:.2} km", orbit.altitude_km);
            println!("Radius: {:.2} km", orbit.radius_km());
            println!("Velocity: {:.3} km/s", orbit.velocity_km_s());
            println!("Period: {:.2} min", orbit.period_minutes());
            println!("Orbits/day: {:.2}", orbit.orbits_per_day());
            println!(
                "Specific energy: {:.3} km²/s²",
                orbit.specific_energy_km2_s2()
            );
        }
    }
}
