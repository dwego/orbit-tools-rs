use std::fs::File;

use csv::Writer;

use crate::signal::LinkBudget;

fn create_writer(filename: &str) -> csv::Result<Writer<File>> {
    Writer::from_path(filename)
}

pub fn create_csv_struct_link_budget(filename: &str) -> csv::Result<Writer<File>>{
    let mut writer = create_writer(filename)?;
    writer.write_record(["distance_km",
                            "frequency_mhz",
                            "tx_power_dbm",
                            "tx_gain-dbi",
                            "rx_gain_dbi",
                            "wavelength_m",
                            "quarter_wave_antenna_m",
                            "free_space_path_loss_db",
                            "EIRP_dbm",
                            "received_power_dbm",
                        ], )?;
    Ok(writer)
}
pub fn write_signal_report(
    writer: &mut Writer<File>,
    link_budget: &LinkBudget,
) -> csv::Result<()> {
    writer.write_record([
        &format!("{:.2}", link_budget.frequency_mhz),
        &format!("{:.2}", link_budget.distance_km),
        &format!("{:.2}", link_budget.tx_power_dbm),
        &format!("{:.2}", link_budget.tx_gain_dbi),
        &format!("{:.2}", link_budget.rx_gain_dbi),
        &format!("{:.2}", link_budget.wavelength_m()),
        &format!("{:.2}", link_budget.quarter_wave_antenna_m()),
        &format!("{:.2}", link_budget.free_space_path_loss_db()),
        &format!("{:.2}", link_budget.eirp_dbm()),
        &format!("{:.2}", link_budget.received_power_dbm()),
    ])?;

    Ok(())
}

pub fn print_signal_report(
    link_budget: &LinkBudget,
) {
    println!("Signal parameters");
    println!("Distance: {:.2} km", link_budget.distance_km);
    println!("Frequency: {:.2} mhz", link_budget.frequency_mhz);
    println!("Tx Power: {:.3} dBm", link_budget.tx_power_dbm);
    println!("Tx Gain: {:.3} dBi", link_budget.tx_gain_dbi);
    println!("Rx Gain: {:.3} dBi", link_budget.rx_gain_dbi);
    println!("Wavelength: {:.3} m", link_budget.wavelength_m());
    println!("Quarter Wave Antenna: {:.3} m", link_budget.quarter_wave_antenna_m());
    println!("Free Space Path Loss: {:.3} dB", link_budget.free_space_path_loss_db());
    println!("EIRP: {:.2} dBm,", link_budget.eirp_dbm());
    println!("Received Power: {:.2} dBm", link_budget.received_power_dbm());
}

pub fn link_budget_filename(
    link_budget: &LinkBudget,
) -> String {
    format!(
        "link-budget-f{:.0}mhz-d{:.0}km-tx{:.0}dbm-gtx{:.0}dbi-grx{:.0}dbi.csv",
        link_budget.frequency_mhz,
        link_budget.distance_km,
        link_budget.tx_power_dbm,
        link_budget.tx_gain_dbi,
        link_budget.rx_gain_dbi,
    )
}