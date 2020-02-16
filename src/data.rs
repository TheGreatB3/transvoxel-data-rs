pub struct RegularCellData {
    geometry_counts: u8,
    vertex_index: [u8; 15],
}

impl RegularCellData {
    pub fn get_vertex_count(&self) -> u8 {
        self.geometry_counts >> 4
    }

    pub fn get_triangle_count(&self) -> u8 {
        self.geometry_counts & 0x0F
    }
}
