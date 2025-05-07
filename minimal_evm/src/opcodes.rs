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
    pub const SIGNEXTEND: u8 =0xb;
    pub const LT: u8 =0x10;
    pub const GT: u8 =0x11;
    pub const SLT: u8 =0x12;
    pub const SGT: u8 =0x13;
    pub const EQ: u8 =0x14;
    pub const ISZERO: u8 =0x15;
    pub const AND: u8 =0x16;
    pub const OR: u8 =0x17;
    pub const XOR: u8 =0x18;
    pub const NOT: u8 =0x19;
    pub const BYTE: u8 =0x1a;
    pub const SHL: u8 =0x1b;
    pub const SHR: u8 =0x1c;
    pub const SAR: u8 =0x1d;
    pub const POP: u8 =0x50;
    pub const MLOAD: u8 =0x51;
    pub const MSTORE: u8 =0x52;
    pub const MSTORE8: u8 =0x53;
    pub const MSIZE: u8 =0x59;
    pub const PUSH0: u8 =0x5f;
    pub const PUSH32: u8 =0x7f;
}
