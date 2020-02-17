extern crate noise;

mod data;
pub mod regular_cell_data;
pub mod transition_cell_data;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
