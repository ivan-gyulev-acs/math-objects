use std::{
	ops::{Add, AddAssign, Sub, SubAssign, Index, IndexMut},
	convert::From,
	fmt::{self, Display, Formatter}
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector<Type, const COUNT: usize> {
	values: [Type; COUNT]
}

impl<Type, const COUNT: usize> Index<usize> for Vector<Type, COUNT> {
	type Output = Type;
	fn index(&self, index: usize) -> &Self::Output {
		&self.values[index]
	}
}

impl<Type, const COUNT: usize> IndexMut<usize> for Vector<Type, COUNT> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.values[index]
	}
}

impl<Type: Copy, const COUNT: usize> From<[Type; COUNT]> for Vector<Type, COUNT> {
	fn from(values: [Type; COUNT]) -> Self {
		Self { values: values.clone() }
	}
}

#[macro_export]
macro_rules! vector {
	[$($items:expr),*] => (Vector::from([$($items),*]))
}

impl<Type: Copy + AddAssign, const COUNT: usize> Add for Vector<Type, COUNT> {
    type Output = Self;
    fn add(self, add: Self) -> Self {
        let mut output = self;
		for index in 0..COUNT {
			output[index] += add[index];
		}
		output
    }
}

impl<Type: Copy + AddAssign, const COUNT: usize> AddAssign for Vector<Type, COUNT> {
    fn add_assign(&mut self, add: Self) {
		for index in 0..COUNT {
			self[index] += add[index];
		}
    }
}

impl<Type: Copy + SubAssign, const COUNT: usize> Sub for Vector<Type, COUNT> {
    type Output = Self;
    fn sub(self, subtract: Self) -> Self {
        let mut output = self;
		for index in 0..COUNT {
			output[index] -= subtract[index];
		}
		output
    }
}

impl<Type: Copy + SubAssign, const COUNT: usize> SubAssign for Vector<Type, COUNT> {
    fn sub_assign(&mut self, subtract: Self) {
		for index in 0..COUNT {
			self[index] -= subtract[index];
		}
    }
}

impl<Type: Display, const COUNT: usize> Display for Vector<Type, COUNT> {
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
		if COUNT == 0 { return write!(formatter, "()"); }
		write!(formatter, "({}", self.values[0])?;
		for item in &self.values[1..] {
			write!(formatter, "; {}", item)?;
		}
		write!(formatter, ")")
	}
}