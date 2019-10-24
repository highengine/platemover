use std::f64::consts;

const DEGREE : f64 = consts::PI/180.;
const RADIAN : f64 = 1.;

const KELVIN : f64 = 1.;

const MICROGRAM : f64 = 1e-9;                        // kilograms
const MILLIGRAM : f64 = 1e-6;                        // kilograms
const GRAM : f64 = 1e-3;                            // kilograms
const KILOGRAM : f64 = 1.;                            // kilograms
const TON : f64 = 1000.;                            // kilograms

const NANOMETER : f64 = 1e-9;                        // meter
const MICROMETER : f64 = 1e-6;                    // meter
const MILLIMETER : f64 = 1e-3;                    // meter
const METER : f64 = 1.;                            // meter
const KILOMETER : f64 = 1000.;                        // meter

const MOLE : f64 = 6.02214076e23;
const MILLIMOLE : f64 = MOLE / 1e3;
const MICROMOLE : f64 = MOLE / 1e6;
const NANOMOLE : f64 = MOLE / 1e9;
const FEMTOMOLE : f64 = MOLE / 1e12;

const SECOND : f64 = 1.;                            // seconds
const MINUTE : f64 = 60.;                            // seconds
const HOUR : f64 = MINUTE*60.;                // seconds
const DAY : f64 = HOUR*24.;                    // seconds
const WEEK : f64 = DAY*7.;                    // seconds
const MONTH : f64 = DAY*29.53059;            // seconds
const YEAR : f64 = DAY*365.256363004;        // seconds
const MEGAYEAR : f64 = YEAR*1e6;            // seconds

const NEWTON : f64 = KILOGRAM *  METER / ( SECOND * SECOND);
const JOULE : f64 = NEWTON *  METER;
const WATT : f64 = JOULE / SECOND;

const EARTH_MASS : f64 = 5.972e24;                 // kilograms
const EARTH_RADIUS : f64 = 6.367e6;                 // meters
const STANDARD_GRAVITY : f64 = 9.80665;             // meters/second^2
const STANDARD_TEMPERATURE : f64 = 273.15;         // kelvin
const STANDARD_PRESSURE : f64 = 101325.;             // pascals
const ASTRONOMICAL_UNIT : f64 = 149597870700.;     // meters
const GLOBAL_SOLAR_CONSTANT : f64 = 1361.;             // watts/meter^2

const JUPITER_MASS : f64 = 1.898e27;                 // kilograms
const JUPITER_RADIUS : f64 = 71e6;                 // meters

const SOLAR_MASS : f64 = 2e30;                     // kilograms
const SOLAR_RADIUS : f64 = 695.7e6;                 // meters
const SOLAR_LUMINOSITY : f64 = 3.828e26;             // watts
const SOLAR_TEMPERATURE : f64 = 5772.;              // kelvin