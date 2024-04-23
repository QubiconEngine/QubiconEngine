use core::{ str::FromStr, fmt::Display };


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pack {
    P8,
    P16,
    P32,
    Block
}

impl FromStr for Pack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PACK8" => Ok(Self::P8),
            "PACK16" => Ok(Self::P16),
            "PACK32" => Ok(Self::P32),
            "BLOCK" => Ok(Self::Block),

            _ => Err(())
        }
    }
}

impl Display for Pack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::P8 => "Pack8",
                Self::P16 => "Pack16",
                Self::P32 => "Pack32",
                Self::Block => "Block"
            }
        )
    }
}



#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Unorm,
    Snorm,
    UScaled,
    SScaled,
    UInt,
    SInt,
    SFloat,
    SRGB
}

impl FromStr for Space {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "UNORM" => Self::Unorm,
            "SNORM" => Self::Snorm,
            "USCALED" => Self::UScaled,
            "SSCALED" => Self::SScaled,
            "UINT" => Self::UInt,
            "SINT" => Self::SInt,
            "SFLOAT" => Self::SFloat,
            "SRGB" => Self::SRGB,

            _ => return Err(())
        };

        Ok(result)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unorm => "Unorm",
                Self::Snorm => "Snorm",
                Self::UScaled => "UScaled",
                Self::SScaled => "SScaled",
                Self::UInt => "UInt",
                Self::SInt => "SInt",
                Self::SFloat => "SFloat",
                Self::SRGB => "SRGB"
            }
        )
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Channel {
    Alpha,
    Red,
    Green,
    Blue,
    Depth,
    Stencil,
    Exponent
}

impl TryFrom<char> for Channel {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Alpha),
            'R' => Ok(Self::Red),
            'G' => Ok(Self::Green),
            'B' => Ok(Self::Blue),
            'D' => Ok(Self::Depth),
            'S' => Ok(Self::Stencil),
            'E' => Ok(Self::Exponent),

            _ => Err(())
        }
    }
}

impl FromStr for Channel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.chars().next().unwrap_or('\n'))
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alpha => "A",
                Self::Red => "R",
                Self::Green => "G",
                Self::Blue => "B",
                Self::Depth => "D",
                Self::Stencil => "S",
                Self::Exponent => "E"
            }
        )
    }
}