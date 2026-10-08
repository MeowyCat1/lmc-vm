use crate::numbers::LMCInt;

pub mod interpreter;
pub mod numbers;

fn add(left : LMCInt, right : LMCInt) -> LMCInt {
    left + right
}

fn sub(left : LMCInt, right : LMCInt) -> LMCInt {
    left - right
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
            assert_eq!(add(LMCInt::new(500), LMCInt::new(-6)), LMCInt::new(494));
            assert_eq!(add(LMCInt::new(-999), LMCInt::new(-2)), LMCInt::new(998));
        }
        #[test]
        fn test_normal_sub() {
        assert_eq!(sub(LMCInt::new(5), LMCInt::new(6)), LMCInt::new(-1));
        assert_eq!(sub(LMCInt::new(500), LMCInt::new(256)), LMCInt::new(244))
        }
        #[test]
        fn test_tricky_sub() {
            assert_eq!(sub(LMCInt::new(5), LMCInt::new(-3)), LMCInt::new(8));
            assert_eq!(sub(LMCInt::new(5), LMCInt::new(7)), LMCInt::new(-2));
            assert_eq!(sub(LMCInt::new(-999), LMCInt::new(3)), LMCInt::new(997));
            assert_eq!(sub(LMCInt::new(999), LMCInt::new(-1)), LMCInt::new(-999));
        }
    }

}