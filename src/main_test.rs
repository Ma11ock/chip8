#![allow(arithmetic_overflow)]


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
    fn sner_test() -> Result<(), InstructionError> {
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

#[cfg(test)]
#[path = "main.rs"]
mod emulate_tests {
    use super::super::{*};
    use std::fmt;
    type I = Instruction;

    fn print_screen_int(screen: &[bool; 64]) -> [u8; 64] {
        let mut result: [u8; 64] = [0; 64];
        for (i, b) in screen.iter().enumerate() {
            result[i] = *b as u8;
        }
        result
    }

    impl fmt::Debug for InterpreterData {
        // Write instruction.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "v:{:?}, i:{}, pc:{}, sp:{}, stack:{:?}, delay:{}, sound:{}",
                   self.v, self.i, self.pc, self.sp, self.stack, self.delay_timer, self.sound_timer);
            // Print the screen.
            write!(f, "\nscreen:");
            for s in self.screen {
                write!(f, "{:?}\n", print_screen_int(&s));
            }
            // Print the memory.
            const STEP: usize = 32;
            write!(f, "\nmem:");
            for s in (0..self.mem.len()).step_by(STEP) {
                write!(f, "{:?}\n", &self.mem[s..s+STEP]);
            }
            write!(f, "\n")
        }
    }

    impl PartialEq for InterpreterData {
        fn eq(&self, other: &Self) -> bool {
            self.v == other.v && self.i == other.i && self.pc == other.pc &&
                self.sp == other.sp && self.stack == other.stack &&
                self.delay_timer == other.delay_timer &&
                self.sound_timer == other.sound_timer &&
                self.mem == other.mem
        }
    }

    #[test]
    fn sys_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate(&vec![I::Sys(0xdef)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.pc += 1;
            e
        });
        Ok(())
    }

    #[test]
    fn cls_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate(&vec![I::Cls], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.pc += 1;
            e
        });
        Ok(())
    }
    
    #[test]
    fn call_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate(&vec![I::Call(0xdef)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.pc = 0xdef;
            e.sp += 1;
            e
        });
        Ok(())
    }

    #[test]
    fn ret_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Call(0x1), I::Ret], &mut emu_state);
        assert_eq!(emu_state, InterpreterData::new());
        Ok(())
    }

    #[test]
    fn jp_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate(&vec![I::Jp(0xdef)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.pc = 0xdef;
            e
        });
        Ok(())
    }

    #[test]
    fn se_test_neq() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 2), I::Se(0, 1)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 2;
            e.pc = 2;
            e
        });
        Ok(())
    }

    #[test]
    fn se_test_eq() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 2), I::Se(0, 2)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 2;
            e.pc = 3;
            e
        });
        Ok(())
    }

    #[test]
    fn sne_test_neq() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 2), I::Sne(0, 1)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 2;
            e.pc = 3;
            e
        });
        Ok(())
    }

    #[test]
    fn sne_test_eq() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 2), I::Sne(0, 2)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 2;
            e.pc = 2;
            e
        });
        Ok(())
    }

    #[test]
    fn ser_test_neq() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 2), I::Ld(1, 1), I::SeR(0, 1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 2;
            e.v[1] = 1;
            e.pc = 3;
            e
        });
        Ok(())
    }

    // Test with two registers that have the same value, should skip
    // 3rd instruction.
    #[test]
    fn ser_test_eq() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 1), I::Ld(1, 1), I::SeR(0, 1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 1;
            e.v[1] = 1;
            e.pc = 4;
            e
        });
        Ok(())
    }

    // Test with two registers that have the same value, should skip
    // 3rd instruction.
    #[test]
    fn ld_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 1)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 1;
            e.pc = 1;
            e
        });
        Ok(())
    }

    #[test]
    fn add_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 1), I::Add(0, 0xde)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 1 + 0xde;
            e.pc = 2;
            e
        });
        Ok(())
    }

    #[test]
    fn ldr_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 1), I::LdR(1, 0)], &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 1;
            e.v[1] = 1;
            e.pc = 2;
            e
        });
        Ok(())
    }

    #[test]
    fn or_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::Ld(1, 0xde), I::Or(0, 1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xde | 0xbe;
            e.v[1] = 0xde;
            e.pc = 3;
            e
        });
        Ok(())
    }

    #[test]
    fn and_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::Ld(1, 0xde), I::And(0, 1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xde & 0xbe;
            e.v[1] = 0xde;
            e.pc = 3;
            e
        });
        Ok(())
    }

    #[test]
    fn xor_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::Ld(1, 0xde), I::Xor(0, 1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xde ^ 0xbe;
            e.v[1] = 0xde;
            e.pc = 3;
            e
        });
        Ok(())
    }

    // AddR with carry.
    #[test]
    fn addr_test_carry() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::Ld(1, 0xde), I::AddR(0, 1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xde + 0xbe;
            e.v[1] = 0xde;
            e.v[0xf] = 1;
            e.pc = 3;
            e
        });
        Ok(())
    }

    // AddR with without carry.
    #[test]
    fn addr_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::Ld(1, 1), I::AddR(0, 1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xbe + 1;
            e.v[1] = 1;
            e.pc = 3;
            e
        });
        Ok(())
    }

    #[test]
    fn ldd_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::LdSD(0), I::LdD(1)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xbe;
            e.delay_timer = 0xbe;
            e.v[1] = 0xbe;
            e.pc = 3;
            e
        });
        Ok(())
    }

    #[test]
    fn ldsd_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::LdSD(0)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xbe;
            e.delay_timer = 0xbe;
            e.pc = 2;
            e
        });
        Ok(())
    }

    #[test]
    fn lds_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xbe), I::LdS(0)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xbe;
            e.sound_timer = 0xbe;
            e.pc = 2;
            e
        });
        Ok(())
    }

    #[test]
    fn addi_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::LdI(0xdef), I::Ld(0, 0xbe), I::AddI(0)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xbe;
            e.i = 0xdef + 0xbe;
            e.pc = 3;
            e
        });
        Ok(())
    }

    #[test]
    fn ldir_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emulate_program(&vec![I::Ld(0, 0xde), I::Ld(1, 0xad),
                              I::Ld(2, 0xbe), I::Ld(3, 0xef), I::Ld(4, 0x69),
                              I::LdI(0x1), I::LdIR(4)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xde;
            e.v[1] = 0xad;
            e.v[2] = 0xbe;
            e.v[3] = 0xef;
            e.v[4] = 0x69;
            
            e.mem[1] = 0xde;
            e.mem[2] = 0xad;
            e.mem[3] = 0xbe;
            e.mem[4] = 0xef;
            // TODO not sure if inclusive.
            e.mem[5] = 0x69;
            e.i = 1;
            e.pc = 7;
            e
        });
        Ok(())
    }

    #[test]
    fn ldirm_test() -> Result<(), InstructionError> {
        let mut emu_state = InterpreterData::new();
        emu_state.mem[1] = 0xde;
        emu_state.mem[2] = 0xad;
        emu_state.mem[3] = 0xbe;
        emu_state.mem[4] = 0xef;
        // TODO not sure if inclusive.
        emu_state.mem[5] = 0x69;
        emulate_program(&vec![I::LdI(1), I::LdIRM(4)],
                        &mut emu_state);
        assert_eq!(emu_state, {
            let mut e = InterpreterData::new();
            e.v[0] = 0xde;
            e.v[1] = 0xad;
            e.v[2] = 0xbe;
            e.v[3] = 0xef;
            e.v[4] = 0xff;

            e.mem[1] = 0xde;
            e.mem[2] = 0xad;
            e.mem[3] = 0xbe;
            e.mem[4] = 0xef;
            e.mem[5] = 0x69;
            e.i = 1;
            e.pc = 2;
            e
        });
        Ok(())
    }
}
