use std::f32::consts::PI;

use crate::graph::constants::MEAN_EARTH_RADIUS_KM;

/// Calculate distance between two nodes given their lat/long in degrees.
pub fn haversine(lat1: f32, long1: f32, lat2: f32, long2: f32) -> f32
{
    let lat1_rad: f32 = lat1 * (PI / 180.0);
    let long1_rad: f32 = long1 * (PI / 180.0);
    let lat2_rad: f32 = lat2 * (PI / 180.0);
    let long2_rad: f32 = long2 * (PI / 180.0);

    let lat_diff: f32 = lat2_rad - lat1_rad;
    let long_diff: f32 = long2_rad - long1_rad;

    let a: f32 = (lat_diff / 2.0).sin().powi(2) + lat1_rad.cos() * 
        lat2_rad.cos() + (long_diff / 2.0).sin().powi(2);
    
    let c: f32 = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    let distance: f32 = MEAN_EARTH_RADIUS_KM * c;

    return distance;
}