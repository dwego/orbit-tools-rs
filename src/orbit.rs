use crate::constants::{EARTH_MU_KM3_S2, EARTH_RADIUS_KM, SECONDS_PER_DAY};

pub struct CircularOrbit {
    pub altitude_km: f64,
}

impl CircularOrbit {
    pub fn new(altitude_km: f64) -> Result<Self, String> {
        if altitude_km <= 0.0 {
            return Err("altitude must be positive".to_string());
        }

        Ok(Self { altitude_km })
    }

    pub fn radius_km(&self) -> f64 {
        EARTH_RADIUS_KM + self.altitude_km
    }

    pub fn velocity_km_s(&self) -> f64 {
        let r = self.radius_km();
        (EARTH_MU_KM3_S2 / r).sqrt()
    }

    pub fn period(&self) -> f64 {
        let semi_major_axis_km = EARTH_RADIUS_KM + self.altitude_km;
        (2.0 * std::f64::consts::PI * semi_major_axis_km.powf(1.5)) / (EARTH_MU_KM3_S2).sqrt()

    }

    pub fn period_minutes(&self) -> f64 {
        self.period() / 60.0
    }

    pub fn orbits_per_day(&self) -> f64 {
        SECONDS_PER_DAY / self.period()
    }

    pub fn specific_energy_km2_s2(&self) -> f64 {
        let r = self.radius_km();
        -EARTH_MU_KM3_S2 / (2.0 * r)
    }

}