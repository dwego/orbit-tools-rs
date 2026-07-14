# orbit-tools-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)


Command-line toolkit for orbital mechanics and RF link-budget calculations, built in Rust as part of the **Orbit Lab** project.

## About Orbit Lab

Orbit Lab is an aerospace engineering project focused on building an accessible, precise, and modular testbed for CubeSat studies. Its goal is to provide a practical platform for simulating, testing, and documenting the key subsystems of a small satellite — attitude control, structure, communication, and orbital analysis.

The project combines physical construction, CAD modeling, embedded electronics, software, applied mathematics, and technical documentation. The aim isn't to build a purely visual CubeSat mockup, but to create a real study and experimentation base that lets contributors validate genuine spacecraft engineering concepts at low cost.

Within Orbit Lab, the physical CubeSat serves as a testbed for design decisions — structure, materials, reaction wheels, control, sensors, power, and communication — while computational tools are developed alongside it to support the calculations and analyses that drive that engineering process.

`orbit-tools-rs` is one of those computational tools.

## What this repository does

This CLI computes and exports orbital and RF communication parameters used during CubeSat mission analysis:

- **Circular orbit mechanics** — given an altitude (or a range of altitudes), computes orbital radius, velocity, escape velocity, period, orbits per day, angular velocity, mean motion, gravitational acceleration, horizon distance, ground footprint radius, horizon angle, and specific orbital energy.
- **RF link budget** — given frequency, distance, transmit power, and antenna gains, computes free-space path loss, EIRP, received power, wavelength, and quarter-wave antenna length.

Results are printed to the terminal and exported to CSV files for further analysis or documentation.

## Usage

### Circular orbit sweep

Sweeps a range of altitudes (in km) and writes one CSV row per altitude.

```bash
cargo run -- circular --start 400 --end 800 --step 50 --out data/orbits.csv
```

| Flag | Description |
|------|-------------|
| `--start` | Starting altitude in km |
| `--end` | Ending altitude in km |
| `--step` | Altitude increment in km |
| `--out` | Output CSV path |

### RF link budget

Computes the link budget for a single set of RF parameters and writes the result to an auto-named CSV file.

```bash
cargo run -- link-budget \
  --frequency-mhz 2400 \
  --distance-km 1000 \
  --tx-power-dbm 30 \
  --tx-gain-dbi 2 \
  --rx-gain-dbi 12
```

| Flag | Description |
|------|-------------|
| `--frequency-mhz` | Carrier frequency in MHz |
| `--distance-km` | Link distance in km |
| `--tx-power-dbm` | Transmit power in dBm |
| `--tx-gain-dbi` | Transmit antenna gain in dBi |
| `--rx-gain-dbi` | Receive antenna gain in dBi |

The output filename encodes the input parameters, e.g. `link-budget-f2400mhz-d1000km-tx30dbm-gtx2dbi-grx12dbi.csv`.
