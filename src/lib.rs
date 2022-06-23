// #![feture(generic_const_exprs)]
#[macro_use]
mod vector;

#[cfg(test)]
mod tests {
	use crate::point::Point;
    #[test]
    fn diplay() {
		let x = 1;
		let vector = vector![x, 2, 3];
        assert_eq!(format!("{}", point), "(1; 2; 3)");
    }
}