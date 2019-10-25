use std::cmp::max;
use std::f64::consts;

extern crate nalgebra as na;

pub const DEGREE: f64 = consts::PI / 180.;
pub const RADIAN: f64 = 1.;

pub const KELVIN: f64 = 1.;

pub const MICROGRAM: f64 = 1e-9;
// kilograms
pub const MILLIGRAM: f64 = 1e-6;
// kilograms
pub const GRAM: f64 = 1e-3;
// kilograms
pub const KILOGRAM: f64 = 1.;
// kilograms
pub const TON: f64 = 1000.;                            // kilograms

pub const NANOMETER: f64 = 1e-9;
// meter
pub const MICROMETER: f64 = 1e-6;
// meter
pub const MILLIMETER: f64 = 1e-3;
// meter
pub const METER: f64 = 1.;
// meter
pub const KILOMETER: f64 = 1000.;                        // meter

pub const MOLE: f64 = 6.02214076e23;
pub const MILLIMOLE: f64 = MOLE / 1e3;
pub const MICROMOLE: f64 = MOLE / 1e6;
pub const NANOMOLE: f64 = MOLE / 1e9;
pub const FEMTOMOLE: f64 = MOLE / 1e12;

pub const SECOND: f64 = 1.;
// seconds
pub const MINUTE: f64 = 60.;
// seconds
pub const HOUR: f64 = MINUTE * 60.;
// seconds
pub const DAY: f64 = HOUR * 24.;
// seconds
pub const WEEK: f64 = DAY * 7.;
// seconds
pub const MONTH: f64 = DAY * 29.53059;
// seconds
pub const YEAR: f64 = DAY * 365.256363004;
// seconds
pub const MEGAYEAR: f64 = YEAR * 1e6;            // seconds

pub const NEWTON: f64 = KILOGRAM * METER / (SECOND * SECOND);
pub const JOULE: f64 = NEWTON * METER;
pub const WATT: f64 = JOULE / SECOND;

pub const EARTH_MASS: f64 = 5.972e24;
// kilograms
pub const EARTH_RADIUS: f64 = 6.367e6;
// meters
pub const STANDARD_GRAVITY: f64 = 9.80665;
// meters/second^2
pub const STANDARD_TEMPERATURE: f64 = 273.15;
// kelvin
pub const STANDARD_PRESSURE: f64 = 101325.;
// pascals
pub const ASTRONOMICAL_UNIT: f64 = 149597870700.;
// meters
pub const GLOBAL_SOLAR_CONSTANT: f64 = 1361.;             // watts/meter^2

pub const JUPITER_MASS: f64 = 1.898e27;
// kilograms
pub const JUPITER_RADIUS: f64 = 71e6;                 // meters

pub const SOLAR_MASS: f64 = 2e30;
// kilograms
pub const SOLAR_RADIUS: f64 = 695.7e6;
// meters
pub const SOLAR_LUMINOSITY: f64 = 3.828e26;
// watts
pub const SOLAR_TEMPERATURE: f64 = 5772.;              // kelvin

pub const SPEED_OF_LIGHT: f64 = 299792458. * METER / SECOND;
pub const BOLTZMANN_CONSTANT: f64 = 1.3806485279e-23 * JOULE / KELVIN;
pub const STEPHAN_BOLTZMANN_CONSTANT: f64 = 5.670373e-8 * WATT / (METER * METER * KELVIN * KELVIN * KELVIN * KELVIN);
pub const PLANCK_CONSTANT: f64 = 6.62607004e-34 * JOULE * SECOND;

pub fn get_surface_area_of_sphere(radius: f64) -> f64 {
    4. * consts::PI * radius * radius
}

pub fn try_get_relation_between_ray_and_sphere(
    sphere_radius: f64,
    z2: f64,
    xz: f64,
) -> bool {
    let sphere_radius2 = sphere_radius * sphere_radius;

    let distance_from_closest_approach_to_exit = max(sphere_radius2 - z2, 1e-10).sqrt();
    let distance_to_entrance = xz - distance_from_closest_approach_to_exit;
    let distance_to_exit = xz + distance_from_closest_approach_to_exit;

    return distance_to_exit > 0. && z2 < sphere_radius * sphere_radius;
}

pub fn solve_fraction_of_light_emitted_by_black_body_below_wavelength(
    wavelength: f64,
    temperature: f64,
) -> f64 {
    let iterations = 2.;
    let h = PLANCK_CONSTANT;
    let k = BOLTZMANN_CONSTANT;
    let c = SPEED_OF_LIGHT;

    let L = wavelength;
    let T = temperature;

    let C2 = h * c / k;
    let z = C2 / (L * T);
    let z2 = z * z;
    let z3 = z2 * z;

    let sum = 0.;
    let mut n2 = 0.;
    let mut n3 = 0.;
    for bn in 1..iterations {
        let n = bn as f64;
        n2 = n * n;
        n3 = n2 * n;
        sum += (z3 + 3. * z2 / n + 6. * z / n2 + 6. / n3) * exp(-n * z) / n;
    }
    return 15. * sum / (consts::PI * *4);
}
