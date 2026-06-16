use std::fs::File;
use csv::Writer;
use crate::orbit::CircularOrbit;

// MATH UTILS

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

pub fn seconds_in_hour(seconds: f64) -> f64 {
    seconds / 3600.0
}

pub fn seconds_in_minute(seconds: f64) -> f64 {
    seconds / 60.0
}

pub fn seconds_in_days(seconds: f64) -> f64 {
    seconds / 86400.0
}


// DATA UTILS

pub fn create_writer(filename: &str) -> csv::Result<Writer<File>> {
    Writer::from_path(filename)
}

pub fn create_data_file(mut wtr: Writer<File>, orbit: &CircularOrbit) -> csv::Result<()> {
    wtr.write_record(["metric", "value", "unit"])?;

    wtr.write_record([
        "Altitude",
        &format!("{:.2}", orbit.altitude_km),
        "km",
    ])?;

    wtr.write_record([
        "Radius",
        &format!("{:.2}", orbit.radius_km()),
        "km",
    ])?;

    wtr.write_record([
        "Velocity",
        &format!("{:.3}", orbit.velocity_km_s()),
        "km/s",
    ])?;

    wtr.write_record([
        "Escape velocity",
        &format!("{:.3}", orbit.escape_velocity_km_s()),
        "km/s",
    ])?;

    wtr.write_record([
        "Gravity",
        &format!("{:.3}", orbit.gravitational_acceleration_m_s2()),
        "m/s^2",
    ])?;

    wtr.write_record([
        "Period",
        &format!("{:.2}", orbit.period_minutes()),
        "min",
    ])?;

    wtr.write_record([
        "Orbits per day",
        &format!("{:.2}", orbit.orbits_per_day()),
        "rev/day",
    ])?;

    wtr.write_record([
        "Angular velocity",
        &format!("{:.6}", orbit.angular_velocity_rad_s()),
        "rad/s",
    ])?;

    wtr.write_record([
        "Mean motion",
        &format!("{:.2}", orbit.mean_motion_rev_day()),
        "rev/day",
    ])?;

    wtr.write_record([
        "Horizon distance",
        &format!("{:.2}", orbit.horizon_distance_km()),
        "km",
    ])?;

    wtr.write_record([
        "Ground footprint radius",
        &format!("{:.2}", orbit.ground_footprint_radius_km()),
        "km",
    ])?;

    wtr.write_record([
        "Horizon angle",
        &format!("{:.2}", orbit.central_angle_to_horizon_deg()),
        "deg",
    ])?;

    wtr.write_record([
        "Specific energy",
        &format!("{:.3}", orbit.specific_energy_km2_s2()),
        "km^2/s^2",
    ])?;

    wtr.flush()?;

    Ok(())
}

pub fn print_data(orbit: &CircularOrbit) {
    println!("Orbit type: circular");
    println!("Altitude: {:.2} km", orbit.altitude_km);
    println!("Radius: {:.2} km", orbit.radius_km());
    println!("Velocity: {:.3} km/s", orbit.velocity_km_s());
    println!("Escape velocity: {:.3} km/s", orbit.escape_velocity_km_s());
    println!("Gravity: {:.3} m/s²", orbit.gravitational_acceleration_m_s2());
    println!("Period: {:.2} min", orbit.period_minutes());
    println!("Orbits/day: {:.2}", orbit.orbits_per_day());
    println!("Angular velocity: {:.6} rad/s", orbit.angular_velocity_rad_s());
    println!("Mean motion: {:.2} rev/day", orbit.mean_motion_rev_day());
    println!("Horizon distance: {:.2} km", orbit.horizon_distance_km());
    println!("Ground footprint radius: {:.2} km", orbit.ground_footprint_radius_km());
    println!("Horizon angle: {:.2} deg", orbit.central_angle_to_horizon_deg());
    println!(
        "Specific energy: {:.3} km²/s²",
        orbit.specific_energy_km2_s2()
    );
}