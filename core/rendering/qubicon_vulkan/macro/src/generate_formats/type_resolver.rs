use super::*;

pub fn resolve(space: Space, bits: u8) -> Option<&'static str> {
    let result = match (space, bits) {
        (Space::Unorm, 32) => "u32",
        (Space::Unorm, 16) => "u16",
        (Space::Unorm, 8) => "u8",

        (Space::Snorm, 32) => "i32",
        (Space::Snorm, 16) => "i16",
        (Space::Snorm, 8) => "i8",

        (Space::UScaled, 32) => "u32",
        (Space::UScaled, 16) => "u16",
        (Space::UScaled, 8) => "u8",

        (Space::SScaled, 32) => "i32",
        (Space::SScaled, 16) => "i16",
        (Space::SScaled, 8) => "i8",

        (Space::UInt, 32) => "u32",
        (Space::UInt, 16) => "u16",
        (Space::UInt, 8) => "u8",

        (Space::SInt, 32) => "i32",
        (Space::SInt, 16) => "i16",
        (Space::SInt, 8) => "i8",

        (Space::SFloat, 32) => "f32",
        (Space::SRGB, 32) => "f32",

        _ => return None
    };

    Some( result )
}