//! Look-up tables used by the Transvoxel Algorithm.
//!
//! This provides the look-up tables needed for the [Transvoxel Algorithm](http://transvoxel.org).
//! The algorithm implementation is not provided in this crate.
//!
//! Based on the work by Eric Lengyel.

pub mod regular_cell_data;
pub mod transition_cell_data;
pub mod prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
