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

    pub fn sweep(start_km: f64, end_km: f64, step_km: f64) -> Result<Vec<Self>, String> {
        if start_km <= 0.0 {
            return Err("start altitude must be positive".to_string());
        }

        if end_km < start_km {
            return Err("end altitude must be greater than or equal to start altitude".to_string());
        }

        if step_km <= 0.0 {
            return Err("step must be positive".to_string());
        }

        let mut orbits = Vec::new();
        let mut altitude_km = start_km;

        while altitude_km <= end_km {
            orbits.push(Self::new(altitude_km)?);
            altitude_km += step_km;
        }

        Ok(orbits)
    }

    pub fn radius_km(&self) -> f64 {
        EARTH_RADIUS_KM + self.altitude_km
    }

    pub fn velocity_km_s(&self) -> f64 {
        let r = self.radius_km();
        (EARTH_MU_KM3_S2 / r).sqrt()
    }

    pub fn escape_velocity_km_s(&self) -> f64 {
        let r = self.radius_km();
        (2.0 * EARTH_MU_KM3_S2 / r).sqrt()
    }

    pub fn period_seconds(&self) -> f64 {
        let a = self.radius_km();

        2.0 * std::f64::consts::PI * (a.powi(3) / EARTH_MU_KM3_S2).sqrt()
    }

    pub fn period_minutes(&self) -> f64 {
        self.period_seconds() / 60.0
    }

    pub fn period_hours(&self) -> f64 {
        self.period_seconds() / 3600.0
    }

    pub fn orbits_per_day(&self) -> f64 {
        SECONDS_PER_DAY / self.period_seconds()
    }

    pub fn angular_velocity_rad_s(&self) -> f64 {
        2.0 * std::f64::consts::PI / self.period_seconds()
    }

    pub fn angular_velocity_deg_s(&self) -> f64 {
        self.angular_velocity_rad_s().to_degrees()
    }

    pub fn mean_motion_rev_day(&self) -> f64 {
        self.orbits_per_day()
    }

    pub fn gravitational_acceleration_km_s2(&self) -> f64 {
        let r = self.radius_km();
        EARTH_MU_KM3_S2 / r.powi(2)
    }

    pub fn gravitational_acceleration_m_s2(&self) -> f64 {
        self.gravitational_acceleration_km_s2() * 1000.0
    }

    pub fn specific_energy_km2_s2(&self) -> f64 {
        let r = self.radius_km();
        -EARTH_MU_KM3_S2 / (2.0 * r)
    }

    pub fn horizon_distance_km(&self) -> f64 {
        let r = self.radius_km();
        (r.powi(2) - EARTH_RADIUS_KM.powi(2)).sqrt()
    }

    pub fn central_angle_to_horizon_rad(&self) -> f64 {
        (EARTH_RADIUS_KM / self.radius_km()).acos()
    }

    pub fn central_angle_to_horizon_deg(&self) -> f64 {
        self.central_angle_to_horizon_rad().to_degrees()
    }

    pub fn ground_footprint_radius_km(&self) -> f64 {
        EARTH_RADIUS_KM * self.central_angle_to_horizon_rad()
    }

    pub fn slant_range_to_horizon_km(&self) -> f64 {
        self.horizon_distance_km()
    }
}