#[derive(Debug, Clone)]
pub struct LinkBudget {
    pub frequency_mhz: f64,
    pub distance_km: f64,
    pub tx_power_dbm: f64,
    pub tx_gain_dbi: f64,
    pub rx_gain_dbi: f64,
}

impl LinkBudget {
    pub fn new(frequency_mhz: f64, distance_km: f64, tx_power_dbm: f64, tx_gain_dbi: f64, rx_gain_dbi:  f64) -> Result<Self, String> {
        if frequency_mhz <= 0.0 {
            return Err("frequency_mhz must be positive".to_string());
        }

        if distance_km <= 0.0 {
            return Err("distance_km must be positive".to_string());
        }

        Ok(LinkBudget { frequency_mhz, distance_km, tx_power_dbm, tx_gain_dbi, rx_gain_dbi })
    }

    pub fn free_space_path_loss_db(&self) -> f64 {
        20.0 * self.distance_km.log10()
            + 20.0 * self.frequency_mhz.log10()
            + 32.44
    }

    pub fn received_power_dbm(&self) -> f64 {
        self.tx_power_dbm + self.tx_gain_dbi + self.rx_gain_dbi - self.free_space_path_loss_db()
    }

    pub fn wavelength_m(&self) -> f64 {
        299_792_458.0 / (self.frequency_mhz * 1_000_000.0)
    }

    pub fn quarter_wave_antenna_m(&self) -> f64 {
        self.wavelength_m() / 4.0
    }
    pub fn eirp_dbm(&self) -> f64 {
        self.tx_power_dbm + self.tx_gain_dbi
    }
}