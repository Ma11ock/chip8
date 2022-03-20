#[cfg(test)]
#[path = "main.rs"]
mod nibble_tests {
    use super::super::{*};

    #[test]
    fn nibble_test() {
        assert_eq!(get_first_nibble(0x8a34u16), 0x4u8);
    }
}


#[cfg(test)]
#[path = "main.rs"]
mod instruction_tests {
    use super::super::{*};
    use std::fmt;
    type I = Instruction;

    impl fmt::Debug for I {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match *self {
                I::Cls => "Cls",
                _ => "Not implemented"
            })
        }
    }

    impl fmt::Debug for InstructionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid instruction")
        }
    }

    #[test]
    fn cls_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x00e0u16)?, I::Cls);
        Ok(())
    }
}
