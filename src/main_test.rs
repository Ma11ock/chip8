#[cfg(test)]
#[path = "main.rs"]
mod nibble_tests {
    use super::super::{*};

    #[test]
    fn nibble_test() {
        assert_eq!(get_first_nibble(0x8a34u16), 0x4u8);
    }

    #[test]
    fn msn_test() {
        assert_eq!(get_fourth_nibble(0xf015u16), 0xf);
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
                I::Sys(nnn) => format!("Sys {}", nnn),
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
    fn sys_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x0defu16)?, I::Sys(0xdef));
        Ok(())
    }

    #[test]
    fn cls_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x00e0u16)?, I::Cls);
        Ok(())
    }

    #[test]
    fn ret_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x00EEu16)?, I::Ret);
        Ok(())
    }

    #[test]
    fn jp_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x1def)?, I::Jp(0xdef));
        Ok(())
    }

    #[test]
    fn call_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x2def)?, I::Call(0xdef));
        Ok(())
    }

    #[test]
    fn se_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x3def)?, I::Se(0xd, 0xef));
        Ok(())
    }

    #[test]
    fn sne_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x4def)?, I::Sne(0xd, 0xef));
        Ok(())
    }

    #[test]
    fn ser_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x5de0)?, I::SeR(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn ld_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x6def)?, I::Ld(0xd, 0xef));
        Ok(())
    }

    #[test]
    fn add_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x7def)?, I::Add(0xd, 0xef));
        Ok(())
    }

    #[test]
    fn ldr_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de0)?, I::LdR(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn or_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de1)?, I::Or(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn and_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de2)?, I::And(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn xor_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de3)?, I::Xor(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn addr_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de4)?, I::AddR(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn sub_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de5)?, I::Sub(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn shr_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de6)?, I::Shr(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn subn_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8de7)?, I::SubN(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn shl_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x8dee)?, I::Shl(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn SneR_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0x9de0)?, I::SneR(0xd, 0x0e));
        Ok(())
    }

    #[test]
    fn ldi_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xadef)?, I::LdI(0xdef));
        Ok(())
    }

    #[test]
    fn jpi_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xbdef)?, I::JpI(0xdef));
        Ok(())
    }

    #[test]
    fn rnd_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xcdef)?, I::Rnd(0xd, 0xef));
        Ok(())
    }

    #[test]
    fn drw_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xddef)?, I::Drw(0xd, 0xe, 0xf));
        Ok(())
    }

    #[test]
    fn skp_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xedee)?, I::Skp(0xd, 0xe));
        Ok(())
    }

    #[test]
    fn skpn_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xede1)?, I::SkpN(0xd, 0xe));
        Ok(())
    }

    #[test]
    fn ldd_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe07)?, I::LdD(0xe));
        Ok(())
    }

    #[test]
    fn ldw_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe0a)?, I::LdW(0xe));
        Ok(())
    }

    #[test]
    fn ldsd_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe15)?, I::LdSD(0xe));
        Ok(())
    }

    #[test]
    fn lds_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe18)?, I::LdS(0xe));
        Ok(())
    }

    #[test]
    fn addi_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe1e)?, I::AddI(0xe));
        Ok(())
    }

    #[test]
    fn ldsp_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe29)?, I::LdSp(0xe));
        Ok(())
    }

    #[test]
    fn ldbcd_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe33)?, I::LdBCD(0xe));
        Ok(())
    }

    #[test]
    fn ldir_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe55)?, I::LdIR(0xe));
        Ok(())
    }

    #[test]
    fn ldirm_test() -> Result<(), InstructionError> {
        assert_eq!(program_to_enum(0xfe65)?, I::LdIRM(0xe));
        Ok(())
    }
}
