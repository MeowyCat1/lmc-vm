use crate::numbers::LMCInt;

pub mod interpreter;
pub mod numbers;

fn add(left : LMCInt, right : LMCInt) -> LMCInt {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_num {
        use super::*;
        #[test]
        fn test_normal_add() {
        assert_eq!(add(LMCInt::new(5), LMCInt::new(6)), LMCInt::new(11));
        assert_eq!(add(LMCInt::new(500), LMCInt::new(256)), LMCInt::new(756))
        }
        #[test]
        fn test_tricky_add() {
            assert_eq!(add(LMCInt::new(998), LMCInt::new(5)), LMCInt::new(-996));
            assert_eq!(add(LMCInt::new(998), LMCInt::new(1)), LMCInt::new(999));
            assert_eq!(add(LMCInt::new(500), LMCInt::new(600)), LMCInt::new(-899));
        }
    }

}