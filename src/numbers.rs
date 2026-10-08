use std::{fmt::Display, ops::{Add, Sub}};

/// A type that stores LMC-compatible numbers and supports rollover of numbers
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LMCInt {pub num : i16}

impl Display for LMCInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl Add for LMCInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let intermediate = self.num + rhs.num;
        if intermediate > 999 {
            Self { num: intermediate - 1999 }
        }
        else if intermediate < -999 {
            Self { num: intermediate + 1999 }
        }
        else {
            Self {num: intermediate}
        }
    }   
}

impl Sub for LMCInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + Self { num: -rhs.num }
    }
}

impl LMCInt {
    pub fn new(num : i16) -> Self {
        Self { num:  num}
    }
}
