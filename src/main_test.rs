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
            // Instruction to string conversion.
            write!(f, "{}", match *self {
                I::Cls => String::from("Cls"),
                I::Ret => String::from("Ret"),
                I::Jp(i) => format!("Jp {}", i),
                I::Call(i) => format!("Call {}", i),
                I::Se(i1, i2) => format!("Se {} {}", i1, i2),
                I::Sne(i1, i2) => format!("Sne {} {}", i1, i2),
                I::SeR(i1, i2) => format!("SeR {} {}", i1, i2),
                I::Ld(i1, i2) => format!("Ld {} {}", i1, i2),
                I::Add(i1, i2) => format!("Add {} {}", i1, i2),
                I::LdR(i1, i2) => format!("LdR {} {}", i1, i2),
                I::Or(i1, i2) => format!("Or {} {}", i1, i2),
                I::And(i1, i2) => format!("And {} {}", i1, i2),
                I::Xor(i1, i2) => format!("Xor {} {}", i1, i2),
                I::AddR(i1, i2) => format!("AddR {} {}", i1, i2),
                I::Sub(i1, i2) => format!("Sub {} {}", i1, i2),
                I::Shr(i1, i2) => format!("Shr {} {}", i1, i2),
                I::SubN(i1, i2) => format!("SubN {} {}", i1, i2),
                I::Shl(i1, i2) => format!("Shl {} {}", i1, i2),
                I::SneR(i1, i2) => format!("SneR {} {}", i1, i2),
                I::LdI(i) => format!("SneR {}", i),
                I::JpI(i) => format!("JpI {}", i),
                I::Rnd(i1, i2) => format!("Rnd {} {}", i1, i2),
                I::Drw(i1, i2, i3) => format!("Rnd {} {} {}", i1, i2, i3),
                I::Skp(i1, i2) => format!("Skp {} {}", i1, i2),
                I::SkpN(i1, i2) => format!("SkpN {} {}", i1, i2),
                I::LdD(i) => format!("LdD {}", i),
                I::LdW(i) => format!("LdW {}", i),
                I::LdS(i) => format!("LdS {}", i),
                I::LdSD(i) => format!("LdSD {}", i),
                I::AddI(i) => format!("AddI {}", i),
                I::LdSp(i) => format!("LdSp {}", i),
                I::LdBCD(i) => format!("LdBCD {}", i),
                I::LdIR(i) => format!("LdIR {}", i),
                I::LdIRM(i) => format!("LdIRM {}", i),
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
