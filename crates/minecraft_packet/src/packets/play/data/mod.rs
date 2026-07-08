mod death_location;
mod position;

pub use death_location::DeathLocation;
pub use position::Position;

pub fn angle_from_degrees(degrees: f32) -> u8 {
    (degrees.rem_euclid(360.0) / 360.0 * 256.0) as u8
}
