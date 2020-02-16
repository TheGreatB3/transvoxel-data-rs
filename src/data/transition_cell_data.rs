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
