pub struct Opcodes;

impl Opcodes{
    pub const ADD: u8 =0x1;
    pub const MUL: u8 =0x2;
    pub const SUB: u8 =0x3;
    pub const DIV: u8 =0x4;
    pub const SDIV: u8 =0x5;
    pub const MOD: u8 =0x6;
    pub const SMOD: u8 =0x7;
    pub const ADDMOD: u8 =0x8;
    pub const MULMOD: u8 =0x9;
    pub const EXP: u8 =0xa;
    pub const POP: u8 = 0x50;
    pub const PUSH0: u8 = 0x5f;
    pub const PUSH32: u8 = 0x7f;
}
