/// Stream Deck Device Kinds
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Kind {
    Original,
    OriginalV2,
    Mini,
    Xl,
    Mk2,
    Plus,
}

/// Stream Deck key layout direction
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyDirection {
    LeftToRight,
    RightToLeft,
}

/// Stream Deck Image Modes
#[derive(Debug, Clone, PartialEq)]
pub enum ImageMode {
    Bmp,
    Jpeg,
}

/// Stream Deck color mode
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ColourOrder {
    RGB,
    BGR,
}

/// Rotation to apply to an image
#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

/// Mirroring to apply to an image
#[derive(Debug, Clone, PartialEq)]
pub enum Mirroring {
    None,
    X,
    Y,
    Both,
}

impl Kind {
    pub fn keys(&self) -> u8 {
        match self {
            Kind::Original | Kind::OriginalV2 | Kind::Mk2 => 15,
            Kind::Mini => 6,
            Kind::Xl => 32,
            Kind::Plus => 8,
        }
    }

    // Offset for the first key in button report
    pub(crate) fn key_data_offset(&self) -> usize {
        match self {
            Kind::Original => 0,
            Kind::OriginalV2 | Kind::Mk2 | Kind::Plus => 3,
            Kind::Mini => 0,
            Kind::Xl => 3,
        }
    }

    pub(crate) fn key_direction(&self) -> KeyDirection {
        match self {
            Kind::Original => KeyDirection::RightToLeft,
            _ => KeyDirection::LeftToRight,
        }
    }

    pub(crate) fn key_columns(&self) -> u8 {
        match self {
            Kind::Mini => 3,
            Kind::Original | Kind::OriginalV2 | Kind::Mk2 => 5,
            Kind::Xl => 8,
            Kind::Plus => 4,
        }
    }

    pub(crate) fn dials(&self) -> u8 {
        match self {
            Kind::Plus => 4,
            _ => 0,
        }
    }

    pub fn image_mode(&self) -> ImageMode {
        match self {
            Kind::Original | Kind::Mini => ImageMode::Bmp,
            Kind::OriginalV2 | Kind::Xl | Kind::Mk2 | Kind::Plus => ImageMode::Jpeg,
        }
    }

    pub fn image_size(&self) -> (usize, usize) {
        match self {
            Kind::Original | Kind::OriginalV2 | Kind::Mk2 => (72, 72),
            Kind::Mini => (80, 80),
            Kind::Xl => (96, 96),
            Kind::Plus => (120, 120),
        }
    }

    pub fn image_rotation(&self) -> Rotation {
        match self {
            Kind::Mini => Rotation::Rot270,
            _ => Rotation::Rot0,
        }
    }

    pub fn image_mirror(&self) -> Mirroring {
        match self {
            // Mini has rotation, not mirror
            Kind::Mini => Mirroring::None,
            // On the original the image is flipped across the Y axis
            Kind::Original => Mirroring::Y,
            // On the V2 devices, both X and Y need to flip
            Kind::OriginalV2 | Kind::Xl | Kind::Mk2 => Mirroring::Both,
            // Plus does not need mirroring at all
            Kind::Plus => Mirroring::None,
        }
    }

    pub fn image_size_bytes(&self) -> usize {
        let (x, y) = self.image_size();
        x * y * 3
    }

    pub(crate) fn image_report_len(&self) -> usize {
        match self {
            Kind::Original => 8191,
            _ => 1024,
        }
    }

    pub(crate) fn image_report_header_len(&self) -> usize {
        match self {
            Kind::Original | Kind::Mini => 16,
            Kind::OriginalV2 | Kind::Xl | Kind::Mk2 | Kind::Plus => 8,
        }
    }

    pub fn image_base(&self) -> &[u8] {
        match self {
            // BMP headers for the original and mini
            Kind::Original => &ORIGINAL_IMAGE_BASE,
            Kind::Mini => &MINI_IMAGE_BASE,

            Kind::OriginalV2 | Kind::Xl | Kind::Mk2 | Kind::Plus => &[],
        }
    }

    pub(crate) fn image_colour_order(&self) -> ColourOrder {
        match self {
            Kind::Original | Kind::Mini => ColourOrder::BGR,
            Kind::OriginalV2 | Kind::Xl | Kind::Mk2 | Kind::Plus => ColourOrder::RGB,
        }
    }

    pub(crate) fn is_v2(&self) -> bool {
        match self {
            Kind::OriginalV2 | Kind::Xl | Kind::Mk2 | Kind::Plus => true,
            _ => false,
        }
    }
}

pub const ORIGINAL_IMAGE_BASE: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const MINI_IMAGE_BASE: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
