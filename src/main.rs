use clap::{Args, Parser, Subcommand};
use orbit_tools_rs::orbit::CircularOrbit;
use orbit_tools_rs::output::{create_csv_struct_circular_orbit, print_circular_orbit_report, write_circular_orbit_report};
use orbit_tools_rs::output::signal_report::{create_csv_struct_link_budget, link_budget_filename, print_signal_report, write_signal_report};
use orbit_tools_rs::signal::LinkBudget;
#[derive(Args, Debug, Clone)]
struct AltitudeSweep {
    #[arg(long)]
    start: f64,

    #[arg(long)]
    end: f64,

    #[arg(long)]
    step: f64,

    #[arg(long)]
    out: String,
}

#[derive(Parser, Debug)]
#[command(about = "Small orbital mechanics toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Circular {
        #[command(flatten)]
        altitude_sweep: AltitudeSweep,
    },
    LinkBudget {
        #[arg(long)]
        frequency_mhz: f64,

        #[arg(long)]
        distance_km: f64,

        #[arg(long)]
        tx_power_dbm: f64,

        #[arg(long)]
        tx_gain_dbi: f64,

        #[arg(long)]
        rx_gain_dbi: f64,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Circular {
            altitude_sweep
        } => {

            let orbits = match CircularOrbit::sweep(
                altitude_sweep.start,
                altitude_sweep.end,
                altitude_sweep.step,
            ) {
                Ok(orbits) => orbits,
                Err(message) => {
                    eprintln!("error: {}", message);
                    std::process::exit(1);
                }
            };

            let mut writer = match create_csv_struct_circular_orbit(&*altitude_sweep.out) {
                Ok(writer) => writer,
                Err(error) => {
                    eprintln!("error: failed to create CSV writer: {}", error);
                    std::process::exit(1);
                }
            };

            for orbit in &orbits {
                write_circular_orbit_report(&mut writer, &orbit).expect("failed to write orbit data");
                print_circular_orbit_report(&orbit);
            }

            writer.flush().expect("failed to flush writer");
            println!("CSV written to {}", altitude_sweep.out);
        }
        Commands::LinkBudget {
            frequency_mhz,
            distance_km,
            tx_power_dbm,
            tx_gain_dbi,
            rx_gain_dbi,
        } => {
            let link_budget = match LinkBudget::new(frequency_mhz, distance_km, tx_power_dbm, tx_gain_dbi, rx_gain_dbi) {
                Ok(link_budget) => link_budget,
                Err(message) => {
                    eprintln!("error: {}", message);
                    std::process::exit(1);
                }
            };

            let mut writer = match create_csv_struct_link_budget(&*link_budget_filename(&link_budget)) {
                Ok(writer) => writer,
                Err(error) => {
                    eprintln!("error: failed to create CSV writer: {}", error);
                    std::process::exit(1);
                }
            };
            write_signal_report(&mut writer, &link_budget).expect("failed to write orbit data");
            print_signal_report(&link_budget);
        }
    }
}
