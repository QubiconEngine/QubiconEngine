use core::{ str::FromStr, fmt::Display };


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pack {
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
pub enum Space {
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
pub enum ChannelType {
    Alpha,
    Red,
    Green,
    Blue,
    Depth,
    Stencil,
    Exponent
}

impl TryFrom<char> for ChannelType {
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

impl FromStr for ChannelType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.chars().next().unwrap_or('\n'))
    }
}

impl Display for ChannelType {
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



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Channel {
    pub ty: ChannelType,
    pub bits: u8
}

impl TryFrom<(char, &str)> for Channel {
    type Error = ();

    fn try_from((ty, bits): (char, &str)) -> Result<Self, Self::Error> {
        let ty = ty.try_into()?;
        let bits = bits.parse().map_err(| _ | ())?;

        Ok ( Self { ty, bits } )
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.ty, self.bits)
    }
}



#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ChannelList {
    channels: Vec<Channel>
}

impl FromStr for ChannelList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut channels = Vec::new();

        let types = s.chars().filter(| c | c.is_ascii_alphabetic());
        let bits = s.split(| c: char | c.is_ascii_alphabetic()).filter(| s | !s.is_empty());

        for channel_data in types.zip(bits) {
            channels.push(channel_data.try_into()?);
        }

        Ok ( Self { channels } )
    }
}

impl Display for ChannelList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for channel in self.channels.iter() {
            write!(f, "{}", channel)?
        }

        Ok(())
    }
}