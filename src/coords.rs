use std::{fmt, ops::{Add, AddAssign, Sub, SubAssign}};

/// An axial coordinate pair.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PointAxial(pub isize, pub isize);

impl Add for PointAxial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        PointAxial(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for PointAxial {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for PointAxial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        PointAxial(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for PointAxial {
    fn sub_assign(&mut self, rhs: Self) {
       self.0 -= rhs.0;
       self.1 -= rhs.1;
    }
}

impl fmt::Display for PointAxial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
