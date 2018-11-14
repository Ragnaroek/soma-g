extern crate somag;

use somag::z80;

#[test]
fn test_nop() {
    let s = exec(z80::nop);
    assert_eq!(s, state_no_mem());
}

#[test]
fn test_jp() {
    let mem = [0x3C, 0xAA, 0xFF].to_vec();
    let mut s = exec_mem(mem.clone(), z80::jp);
    assert_eq!(s.reg.pc, 0xFFAA);

    s.reg.pc = 0x0;
    assert_eq!(s, state_mem(mem.clone()));
}

#[test]
fn test_inc_a() {
    let mut s = exec(z80::inc_a);
    assert_eq!(s.reg.a, 1);
    z80::inc_a(&mut s);
    assert_eq!(s.reg.a, 2);

    s.reg.a = 0;
    assert_eq!(s, state_no_mem());
}

#[test]
fn test_inc_l() {
    let mut s = exec(z80::inc_l);
    assert_eq!(s.reg.l, 1);
    z80::inc_l(&mut s);
    assert_eq!(s.reg.l, 2);

    s.reg.l = 0;
    assert_eq!(s, state_no_mem());
}

#[test]
fn test_ld_bc_a() {
    let mut s = state_no_mem();
    s.reg.a = 0x66;
    z80::ld_bc_a(&mut s);

    assert_eq!(s.reg.b, 0x66);

    s.reg.a = 0;
    s.reg.b = 0;
    assert_eq!(s, state_no_mem());
}


#[test]
fn test_u16_le() {
    let mem = [0x3C, 0x50, 0x01];
    let result = z80::u16_le(0, &mem);
    assert_eq!(result, 0x0150);
}

//helper methods

fn state_no_mem() -> z80::State {
    z80::initial_state(Vec::new(), 0x0)
}

fn state_mem(mem: Vec<u8>) -> z80::State {
    z80::initial_state(mem, 0x0)
}

fn exec(effect: fn(&mut z80::State)) -> z80::State {
    let mut s = state_no_mem();
    (effect)(&mut s);
    return s;
}

fn exec_mem(mem: Vec<u8>, effect: fn(&mut z80::State)) -> z80::State {
    let mut s = z80::initial_state(mem, 0x0);
    (effect)(&mut s);
    return s;
}
