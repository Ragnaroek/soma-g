
use std::collections::HashMap;
use std::fmt;

pub fn start(mut state: State) -> Result<(), String> {

    let instr_set = instruction_set();

    loop {
        let op = state.mem[state.reg.pc as usize];
        let instr_lookup = instr_set.get(op);
        if instr_lookup.is_none() {
            return Err(format!("not an instruction: 0x{:x}", op));
        }
        let instr = instr_lookup.unwrap();
        (instr.effect)(&mut state);
        println!("instr = {:?}", instr);
        state.reg.pc = state.reg.pc + 1;
    }
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub mem: Vec<u8>,
    pub stack: Vec<u8>,
    pub reg: Register,
}

#[derive(Debug, PartialEq)]
pub struct Register {
    pub pc: u16,
    pub sp: u16,

    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub l: u8,
    pub f: u8,
}

const Z : u8 = 1 << 7;
const N : u8 = 1 << 6;
const H : u8 = 1 << 5;
const C : u8 = 1 << 4;

impl Register {
    pub fn set_zero_flag(&mut self, b: bool) {
        if b {
            self.f = self.f | Z;
        } else {
            self.f = self.f & !Z;
        }
    }
    pub fn zero_flag(&self) -> bool {
        (self.f & Z) != 0
    }

    pub fn set_carry_flag(&mut self, b: bool) {
        if b {
            self.f = self.f | C;
        } else {
            self.f = self.f & !C;
        }
    }
    pub fn carry_flag(&self) -> bool {
        (self.f & C) != 0
    }

    pub fn set_half_carry_flag(&mut self, b: bool) {
        if b {
            self.f = self.f | H;
        } else {
            self.f = self.f & !H;
        }
    }
    pub fn half_carry_flag(&self) -> bool {
        (self.f & H) != 0
    }

    pub fn set_n_flag(&mut self, b: bool) {
        if b {
            self.f = self.f | N;
        } else {
            self.f = self.f & !N;
        }
    }
    pub fn n_flag(&self) -> bool {
        (self.f & N) != 0
    }
}

pub fn initial_state(mem: Vec<u8>, stack_size: usize, start_pc: u16) -> State {
    return State{mem,
                 stack: vec![0; stack_size],
                 reg: Register{pc: start_pc,
                     sp: stack_size as u16,
                     a: 0,
                     b: 0,
                     c: 0,
                     d: 0,
                     l: 0,
                     f: 0}};
}

struct Instr {
    mnemonic: String,
    op: u8,
    effect: fn(&mut State)
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instr {{ mnemonic: {}, op: {} }}", self.mnemonic, self.op)
    }
}

struct InstrSet {
    set: HashMap<u8, Instr>,
}

impl InstrSet {

    fn new() -> InstrSet {
        let set = HashMap::new();
        return InstrSet{set};
    }

    fn get<'a>(&'a self, op: u8) -> Option<&'a Instr> {
        return self.set.get(&op);
    }

    fn add_instr(&mut self, op: u8, mnemonic: &str, effect: fn(&mut State)) {
        let instr = Instr{op, mnemonic: mnemonic.to_string(), effect};
        self.set.insert(op, instr);
    }
}

fn instruction_set() -> InstrSet {
    let mut instr_set = InstrSet::new();
    instr_set.add_instr(0x00, "NOP", nop);
    instr_set.add_instr(0xC3, "JP", jp);
    instr_set.add_instr(0x0C, "INC A", inc_a);
    instr_set.add_instr(0x2C, "INC L", inc_l);
    instr_set.add_instr(0x02, "LD (BC),A", ld_bc_a);
    instr_set.add_instr(0xCD, "CALL", call);
    instr_set.add_instr(0xB2, "OR D", or_d);
    instr_set.add_instr(0xD6, "SUB byte", sub_byte);
    return instr_set;
}

pub fn read_u16_le(pc: u16, mem: &[u8]) -> u16 {
    let a1 = mem[pc as usize + 2] as u16;
    let a2 = mem[pc as usize + 1] as u16;
    return (a1 << 8) | a2;
}

pub fn read_u8(pc: u16, mem: &[u8]) -> u8 {
    return mem[pc as usize + 1];
}

pub fn u16_reg(reg1: u8, reg2: u8) -> u16 {
    return ((reg1 as u16) << 8) | reg2 as u16;
}

pub fn nop(_s: &mut State) {}

pub fn jp(s: &mut State) {
    let address = read_u16_le(s.reg.pc, &s.mem);
    s.reg.pc = address;
}

pub fn inc_a(s: &mut State) {
    s.reg.a = s.reg.a + 1;
}

pub fn inc_l(s: &mut State) {
    s.reg.l = s.reg.l + 1;
}

pub fn ld_bc_a(s: &mut State) {
    let ptr = u16_reg(s.reg.b, s.reg.c) as usize;
    s.reg.a = s.mem[ptr];
}

pub fn call(s: &mut State) {
    let pc_p = s.reg.pc + 3;
    s.reg.sp = s.reg.sp - 1;
    s.stack[s.reg.sp as usize] = ((pc_p & 0xFF00) >> 8) as u8;
    s.reg.sp = s.reg.sp - 1;
    s.stack[s.reg.sp as usize] = (pc_p & 0x00FF) as u8;
    s.reg.pc = read_u16_le(s.reg.pc, &s.mem);
}

pub fn or_d(s: &mut State) {
    s.reg.a = s.reg.a | s.reg.d;
}

pub fn sub_byte(s: &mut State) {
    // a - val
    let sub_val = read_u8(s.reg.pc, &s.mem);
    let half_carry = (s.reg.a & 0xF) < (sub_val & 0xF);
    let (a_val, carry) = s.reg.a.overflowing_sub(sub_val);
    s.reg.a = a_val;
    let z = s.reg.a == 0;
    s.reg.set_zero_flag(z);
    s.reg.set_carry_flag(carry);
    s.reg.set_half_carry_flag(half_carry);
    s.reg.set_n_flag(true);
}
