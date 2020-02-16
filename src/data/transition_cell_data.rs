pub struct TransitionCellData {
    geometry_counts: u8,
    vertex_index: [u8; 36],
}

impl TransitionCellData {
    pub fn get_vertex_count(&self) -> u8 {
        self.geometry_counts >> 4
    }

    pub fn get_triangle_count(&self) -> u8 {
        self.geometry_counts & 0x0F
    }
}

pub const TRANSITION_CELL_CLASS: [u8; 512] = [
    0x00, 0x01, 0x02, 0x84, 0x01, 0x05, 0x04, 0x04, 0x02, 0x87, 0x09, 0x8C, 0x84, 0x0B, 0x05, 0x05,
    0x01, 0x08, 0x07, 0x8D, 0x05, 0x0F, 0x8B, 0x0B, 0x04, 0x0D, 0x0C, 0x1C, 0x04, 0x8B, 0x85, 0x85,
    0x02, 0x07, 0x09, 0x8C, 0x87, 0x10, 0x0C, 0x0C, 0x09, 0x12, 0x15, 0x9A, 0x8C, 0x19, 0x90, 0x10,
    0x84, 0x8D, 0x8C, 0x9C, 0x0B, 0x9D, 0x0F, 0x0F, 0x05, 0x1B, 0x10, 0xAC, 0x05, 0x0F, 0x8B, 0x0B,
    0x01, 0x05, 0x87, 0x0B, 0x08, 0x0F, 0x0D, 0x8B, 0x07, 0x10, 0x12, 0x19, 0x8D, 0x9D, 0x1B, 0x0F,
    0x05, 0x0F, 0x10, 0x9D, 0x0F, 0x1E, 0x1D, 0xA1, 0x8B, 0x1D, 0x99, 0x32, 0x0B, 0xA1, 0x8F, 0x94,
    0x04, 0x8B, 0x0C, 0x0F, 0x0D, 0x1D, 0x1C, 0x8F, 0x0C, 0x99, 0x1A, 0x31, 0x1C, 0x32, 0x2C, 0xA7,
    0x04, 0x0B, 0x0C, 0x0F, 0x8B, 0xA1, 0x8F, 0x96, 0x85, 0x8F, 0x90, 0x27, 0x85, 0x94, 0x8B, 0x8A,
    0x02, 0x04, 0x09, 0x05, 0x07, 0x8B, 0x0C, 0x85, 0x09, 0x0C, 0x15, 0x90, 0x8C, 0x0F, 0x10, 0x8B,
    0x87, 0x0D, 0x12, 0x1B, 0x10, 0x1D, 0x99, 0x8F, 0x0C, 0x1C, 0x1A, 0x2C, 0x0C, 0x8F, 0x90, 0x8B,
    0x09, 0x0C, 0x15, 0x10, 0x12, 0x99, 0x1A, 0x90, 0x15, 0x1A, 0x23, 0x30, 0x9A, 0x31, 0x30, 0x19,
    0x8C, 0x1C, 0x9A, 0xAC, 0x19, 0x32, 0x31, 0x27, 0x90, 0x2C, 0x30, 0x29, 0x10, 0xA7, 0x19, 0x24,
    0x84, 0x04, 0x8C, 0x05, 0x8D, 0x0B, 0x1C, 0x85, 0x8C, 0x0C, 0x9A, 0x10, 0x9C, 0x0F, 0xAC, 0x0B,
    0x0B, 0x8B, 0x19, 0x0F, 0x9D, 0xA1, 0x32, 0x94, 0x0F, 0x8F, 0x31, 0xA7, 0x0F, 0x96, 0x27, 0x8A,
    0x05, 0x85, 0x90, 0x8B, 0x1B, 0x8F, 0x2C, 0x8B, 0x10, 0x90, 0x30, 0x19, 0xAC, 0x27, 0x29, 0x24,
    0x05, 0x85, 0x10, 0x0B, 0x0F, 0x94, 0xA7, 0x8A, 0x8B, 0x8B, 0x19, 0x24, 0x0B, 0x8A, 0x24, 0x83,
    0x03, 0x06, 0x0A, 0x8B, 0x06, 0x0E, 0x0B, 0x0B, 0x0A, 0x91, 0x14, 0x8F, 0x8B, 0x17, 0x05, 0x85,
    0x06, 0x13, 0x11, 0x98, 0x0E, 0x1F, 0x97, 0x2B, 0x0B, 0x18, 0x0F, 0x36, 0x0B, 0xAB, 0x05, 0x85,
    0x0A, 0x11, 0x16, 0x8F, 0x91, 0x20, 0x0F, 0x8F, 0x14, 0x22, 0x21, 0x1D, 0x8F, 0x2D, 0x0B, 0x8B,
    0x8B, 0x98, 0x8F, 0xB7, 0x17, 0xAE, 0x8C, 0x0C, 0x05, 0x2F, 0x8B, 0xB5, 0x85, 0xA6, 0x84, 0x04,
    0x06, 0x0E, 0x91, 0x17, 0x13, 0x1F, 0x18, 0xAB, 0x11, 0x20, 0x22, 0x2D, 0x98, 0xAE, 0x2F, 0xA6,
    0x0E, 0x1F, 0x20, 0xAE, 0x1F, 0x33, 0x2E, 0x2A, 0x97, 0x2E, 0xAD, 0x28, 0x2B, 0x2A, 0x26, 0x25,
    0x0B, 0x97, 0x0F, 0x8C, 0x18, 0x2E, 0x37, 0x8C, 0x0F, 0xAD, 0x9D, 0x90, 0x36, 0x28, 0x35, 0x07,
    0x0B, 0x2B, 0x8F, 0x0C, 0xAB, 0x2A, 0x8C, 0x89, 0x05, 0x26, 0x0B, 0x87, 0x85, 0x25, 0x84, 0x82,
    0x0A, 0x0B, 0x14, 0x05, 0x11, 0x97, 0x0F, 0x05, 0x16, 0x0F, 0x21, 0x0B, 0x8F, 0x8C, 0x8B, 0x84,
    0x91, 0x18, 0x22, 0x2F, 0x20, 0x2E, 0xAD, 0x26, 0x0F, 0x37, 0x9D, 0x35, 0x8F, 0x8C, 0x0B, 0x84,
    0x14, 0x0F, 0x21, 0x8B, 0x22, 0xAD, 0x9D, 0x0B, 0x21, 0x9D, 0x9E, 0x8F, 0x1D, 0x90, 0x8F, 0x85,
    0x8F, 0x36, 0x1D, 0xB5, 0x2D, 0x28, 0x90, 0x87, 0x0B, 0x35, 0x8F, 0x34, 0x8B, 0x07, 0x85, 0x81,
    0x8B, 0x0B, 0x8F, 0x85, 0x98, 0x2B, 0x36, 0x85, 0x8F, 0x8F, 0x1D, 0x8B, 0xB7, 0x0C, 0xB5, 0x04,
    0x17, 0xAB, 0x2D, 0xA6, 0xAE, 0x2A, 0x28, 0x25, 0x8C, 0x8C, 0x90, 0x07, 0x0C, 0x89, 0x87, 0x82,
    0x05, 0x05, 0x0B, 0x84, 0x2F, 0x26, 0x35, 0x84, 0x8B, 0x0B, 0x8F, 0x85, 0xB5, 0x87, 0x34, 0x81,
    0x85, 0x85, 0x8B, 0x04, 0xA6, 0x25, 0x07, 0x82, 0x84, 0x84, 0x85, 0x81, 0x04, 0x82, 0x81, 0x80,
];
