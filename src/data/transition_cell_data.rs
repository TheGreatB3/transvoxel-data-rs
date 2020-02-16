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

pub const TRANSITION_CELL_DATA: [TransitionCellData; 56] = [
    TransitionCellData {geometry_counts: 0x00, vertex_index: [0; 36]},
    TransitionCellData {geometry_counts: 0x42, vertex_index: [0, 1, 3, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x31, vertex_index: [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x42, vertex_index: [0, 1, 2, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x53, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x64, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x84, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 6, 4, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x73, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x84, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 7, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x62, vertex_index: [0, 1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x53, vertex_index: [0, 1, 3, 0, 3, 4, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x75, vertex_index: [0, 1, 6, 1, 2, 6, 2, 5, 6, 2, 3, 5, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x84, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x95, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 5, 6, 8, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA6, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 6, 7, 8, 6, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x86, vertex_index: [0, 1, 7, 1, 2, 7, 2, 3, 7, 3, 6, 7, 3, 4, 6, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x95, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x95, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 7, 4, 7, 8, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA4, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xC6, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 7, 5, 6, 7, 8, 9, 10, 8, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x64, vertex_index: [0, 1, 3, 1, 2, 3, 0, 3, 4, 0, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x93, vertex_index: [0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x64, vertex_index: [0, 1, 4, 0, 4, 5, 1, 3, 4, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x97, vertex_index: [0, 1, 8, 1, 7, 8, 1, 2, 7, 2, 3, 7, 3, 4, 7, 4, 5, 7, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xB7, vertex_index: [0, 1, 6, 1, 2, 6, 2, 5, 6, 2, 3, 5, 3, 4, 5, 7, 8, 10, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA6, vertex_index: [0, 1, 6, 1, 2, 6, 2, 5, 6, 2, 3, 5, 3, 4, 5, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xB5, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA6, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 6, 7, 9, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA6, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 5, 6, 9, 6, 8, 9, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x97, vertex_index: [0, 1, 8, 1, 2, 8, 2, 3, 8, 3, 7, 8, 3, 4, 7, 4, 5, 7, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x86, vertex_index: [0, 1, 7, 1, 6, 7, 1, 2, 6, 2, 5, 6, 2, 4, 5, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xC8, vertex_index: [0, 1, 7, 1, 2, 7, 2, 3, 7, 3, 6, 7, 3, 4, 6, 4, 5, 6, 8, 9, 10, 8, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xB7, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 6, 9, 10, 6, 7, 9, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x75, vertex_index: [0, 1, 6, 1, 3, 6, 1, 2, 3, 3, 4, 6, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA6, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 9, 5, 8, 9, 5, 6, 8, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xC4, vertex_index: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x86, vertex_index: [1, 2, 4, 2, 3, 4, 0, 1, 7, 1, 4, 7, 4, 6, 7, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x64, vertex_index: [0, 4, 5, 0, 1, 4, 1, 3, 4, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x86, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 0, 4, 7, 4, 6, 7, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x97, vertex_index: [1, 2, 3, 1, 3, 4, 1, 4, 5, 0, 1, 8, 1, 5, 8, 5, 7, 8, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA6, vertex_index: [0, 1, 3, 1, 2, 3, 4, 5, 9, 5, 8, 9, 5, 6, 8, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xC8, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 6, 7, 11, 7, 10, 11, 7, 8, 10, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x97, vertex_index: [0, 1, 8, 1, 2, 8, 2, 7, 8, 2, 3, 7, 3, 6, 7, 3, 4, 6, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x97, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 0, 4, 8, 4, 7, 8, 4, 5, 7, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xB7, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 6, 7, 10, 7, 9, 10, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA8, vertex_index: [0, 1, 9, 1, 2, 9, 2, 8, 9, 2, 3, 8, 3, 7, 8, 3, 4, 7, 4, 6, 7, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xB9, vertex_index: [0, 1, 7, 1, 6, 7, 1, 2, 6, 2, 5, 6, 2, 3, 5, 3, 4, 5, 0, 7, 10, 7, 9, 10, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA6, vertex_index: [0, 1, 5, 1, 4, 5, 1, 2, 4, 2, 3, 4, 6, 7, 9, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xC6, vertex_index: [0, 1, 5, 1, 2, 5, 2, 4, 5, 2, 3, 4, 6, 7, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xB7, vertex_index: [0, 1, 7, 1, 2, 7, 2, 3, 7, 3, 6, 7, 3, 4, 6, 4, 5, 6, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA8, vertex_index: [1, 2, 3, 1, 3, 4, 1, 4, 6, 4, 5, 6, 0, 1, 9, 1, 6, 9, 6, 8, 9, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xCC, vertex_index: [0, 1, 9, 1, 8, 9, 1, 2, 8, 2, 11, 8, 2, 3, 11, 3, 4, 11, 4, 5, 11, 5, 10, 11, 5, 6, 10, 6, 9, 10, 6, 7, 9, 7, 0, 9]},
    TransitionCellData {geometry_counts: 0x86, vertex_index: [0, 1, 2, 0, 2, 3, 0, 6, 7, 0, 3, 6, 1, 4, 5, 1, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0x97, vertex_index: [0, 1, 4, 1, 3, 4, 1, 2, 3, 2, 5, 6, 2, 6, 3, 0, 7, 8, 0, 4, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA8, vertex_index: [0, 1, 5, 1, 4, 5, 1, 2, 4, 2, 3, 4, 3, 6, 7, 3, 7, 4, 0, 8, 9, 0, 5, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
    TransitionCellData {geometry_counts: 0xA8, vertex_index: [0, 1, 5, 1, 4, 5, 1, 2, 4, 2, 3, 4, 2, 6, 3, 3, 6, 7, 0, 8, 9, 0, 5, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},
];

pub const TRANSITION_CORNER_DATA: [u8; 13] = [
    0x30, 0x21, 0x20, 0x12, 0x40, 0x82, 0x10, 0x81, 0x80, 0x37, 0x27, 0x17, 0x87
];
