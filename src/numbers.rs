use std::{fmt::{Display}, ops::Add};

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
            Self{num : -999 + (intermediate - 1000)}
        }
        else {
            Self { num: intermediate }
        }
    }   
}

impl LMCInt {
    pub fn new(num : i16) -> Self {
        Self { num:  num}
    }
}
