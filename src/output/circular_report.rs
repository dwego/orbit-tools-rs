use std::fs::File;

use csv::Writer;

use crate::orbit::CircularOrbit;

fn create_writer(filename: &str) -> csv::Result<Writer<File>> {
    Writer::from_path(filename)
}

pub fn create_csv_struct_circular_orbit(filename: &str) -> csv::Result<Writer<File>>{
    let mut writer = create_writer(filename)?;
    writer.write_record([
        "altitude_km",
        "radius_km",
        "velocity_km_s",
        "escape_velocity_km_s",
        "gravitational_acceleration_m_s2",
        "orbits_per_day",
        "angular_velocity_rad_s",
        "mean_motion_rev_day",
        "horizon_km",
        "ground_footprint_radius_km",
        "central_angle_to_horizon_deg",
        "specific_energy_km2_s2"])?;
    Ok(writer)
}
pub fn write_circular_orbit_report(
    writer: &mut Writer<File>,
    orbit: &CircularOrbit,
) -> csv::Result<()> {
    writer.write_record([
        &format!("{:.2}", orbit.altitude_km),
        &format!("{:.2}", orbit.radius_km()),
        &format!("{:.3}", orbit.velocity_km_s()),
        &format!("{:.3}", orbit.escape_velocity_km_s()),
        &format!("{:.3}", orbit.gravitational_acceleration_m_s2()),
        &format!("{:.2}", orbit.orbits_per_day()),
        &format!("{:.6}", orbit.angular_velocity_rad_s()),
        &format!("{:.2}", orbit.mean_motion_rev_day()),
        &format!("{:.2}", orbit.horizon_distance_km()),
        &format!("{:.2}", orbit.ground_footprint_radius_km()),
        &format!("{:.2}", orbit.central_angle_to_horizon_deg()),
        &format!("{:.3}", orbit.specific_energy_km2_s2()),
    ])?;

    Ok(())
}

pub fn print_circular_orbit_report(orbit: &CircularOrbit) {
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
    println!(
        "Ground footprint radius: {:.2} km",
        orbit.ground_footprint_radius_km()
    );
    println!(
        "Horizon angle: {:.2} deg",
        orbit.central_angle_to_horizon_deg()
    );
    println!(
        "Specific energy: {:.3} km²/s²",
        orbit.specific_energy_km2_s2()
    );
}