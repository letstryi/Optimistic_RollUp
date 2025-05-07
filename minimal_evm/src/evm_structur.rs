use std::collections::VecDeque;
use crate::opcodes::Opcodes;
use primitive_types::U256;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

#[derive(Debug)]
pub struct EVM {
    stack: VecDeque<U256>,
    memory: Vec<u8>,
    code: Vec<u8>,
    //gas: ,
    //storage: ,
    counter: usize,
    pc: usize,
}

impl EVM {
    pub fn new(code: Vec<u8>)-> Self {
        Self {
            stack: VecDeque::new(),
            memory: Vec::new(),
            code,
            pc: 0,
            counter: 0,
        }
    }
    pub fn run(&mut self)->(){
        while self.counter < self.code.len(){
            println!("Byte: 0x{:02X}", self.code[self.counter]);
            match self.code[self.counter] {
                Opcodes::ADD => self.arithmetic('+'),
                Opcodes::MUL => self.arithmetic('*'),
                Opcodes::SUB => self.arithmetic('-'),
                Opcodes::DIV => self.arithmetic('/'),
                Opcodes::SDIV => self.sdiv(),
                Opcodes::MOD => self.arithmetic('%'),
                Opcodes::SMOD => self.smod(),
                Opcodes::ADDMOD => self.addmod(),
                Opcodes::MULMOD => self.mulmod(),
                Opcodes::EXP => self.exp(),
                Opcodes::SIGNEXTEND => self.signextend(),
                Opcodes::LT => self.comparison("<"),
                Opcodes::GT => self.comparison(">"),
                Opcodes::SLT => self.comparison("s<"),
                Opcodes::SGT => self.comparison("s>"),
                Opcodes::EQ => self.comparison("="),
                Opcodes::ISZERO => self.iszero(),
                Opcodes::AND => self.bitwise("&"),
                Opcodes::OR => self.bitwise("|"),
                Opcodes::XOR => self.bitwise("^"),
                Opcodes::NOT => self.not(),
                Opcodes::BYTE => self.bitwise("byte"),
                Opcodes::SHL => self.bitwise("<<"),
                Opcodes::SHR => self.bitwise(">>"),
                Opcodes::SAR => self.bitwise("sar"),
                Opcodes::POP => self.pop(),
                Opcodes::MLOAD => self.mload(),
                Opcodes::MSTORE | Opcodes::MSTORE8 => self.mstore(),
                Opcodes::MSIZE => self.msize(),
                Opcodes::PUSH0 ..=Opcodes::PUSH32 => self.push(),
                _ => println!("other"),
            }
            self.pc +=1;
        }
     }
     
     fn push(&mut self){
        let n = self.code[self.counter]-Opcodes::PUSH0;
        self.counter += 1;
        let mut nbrtopush : U256 = self.code[self.counter].into();
        for i in 1..n {
            nbrtopush = (nbrtopush << 8) | self.code[self.counter+i as usize].into();
        }
        self.stack.push_front(nbrtopush);
        self.counter += n as usize;
     }
     
     fn pop(&mut self) {
        match self.stack.pop_front() {
            Some(s) => println!("the number we pop is : {}",s),
            None => println!("the satck is vide"),
        }
        self.counter +=1;
     }
     
     fn arithmetic(&mut self, op: char){
        if let Some(values) = self.get_n_elements(2){
            let [a,b] = values.try_into().unwrap();
            let base = BigUint::from(2u32);
            match op{
                '+' => self.stack.push_front(Self::biguint_to_u256((a+b)%base.pow(256))),
                '*' => self.stack.push_front(Self::biguint_to_u256((a*b)%base.pow(256))),
                '-' => {
                    if a>=b {
                        self.stack.push_front(Self::biguint_to_u256((a-b)%base.pow(256)));
                    } else {
                        self.stack.push_front(Self::biguint_to_u256(base.pow(256)-(b-a)));
                    }
                }
                '/' => {
                    if b == BigUint::from(0u32) {
                        self.stack.push_front(U256::zero());
                    } else {
                        self.stack.push_front(Self::biguint_to_u256((a/b)%base.pow(256)));
                    }
                }
                '%' => {
                    if b == BigUint::from(0u32) {
                        self.stack.push_front(U256::zero());
                    } else {
                        self.stack.push_front(Self::biguint_to_u256(a%b));
                    }
                }
                _ => println!("he will never run this ;)"),
            }
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn sdiv(&mut self){
        let base = BigUint::from(2u32);
        if let Some(values) = self.get_n_elements(2){
            let [mut a, mut b] = values.try_into().unwrap();
            if b == BigUint::from(0u32) {
                self.stack.push_front(U256::zero());
            } else {
                if(a.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32) {
                    a = Self::get_two_complement_of(a);
                    if (b.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32) {
                        b = Self::get_two_complement_of(b);
                        self.stack.push_front(Self::biguint_to_u256((a/b)%base.pow(256)))
                    } else {
                        self.stack.push_front(Self::biguint_to_u256((Self::get_two_complement_of(a/b) | BigUint::from(2u32).pow(255)) %base.pow(256)));
                    }
                } else {
                    if (b.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32) {
                        b = Self::get_two_complement_of(b);
                        self.stack.push_front(Self::biguint_to_u256((Self::get_two_complement_of(a/b) | BigUint::from(2u32).pow(255)) %base.pow(256)));
                    } else {
                        self.stack.push_front(Self::biguint_to_u256((a/b)%base.pow(256)))
                    }
                }
            }
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn smod(&mut self){
         if let Some(values) = self.get_n_elements(2){
            let [mut a, mut b] = values.try_into().unwrap();
            if b == BigUint::from(0u32) {
                self.stack.push_front(U256::zero());
            } else {
                if (a.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32){
                    a = Self::get_two_complement_of(a);
                    println!("a is negative : {}",a);
                    if (b.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32){
                        b = Self::get_two_complement_of(b);
                        println!("b is negative : {}",b);
                    }
                    self.stack.push_front(Self::biguint_to_u256(Self::get_two_complement_of(a%b) | BigUint::from(2u32).pow(255)));                    
                } else {
                    if (b.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32){
                        b = Self::get_two_complement_of(b);
                    }
                    self.stack.push_front(Self::biguint_to_u256(a%b));
                }
            }
         } else {
            println!("there is not enough value in stack");
         }
         self.counter +=1;
     }
     
     fn addmod(&mut self){
        self.arithmetic('+');
        self.arithmetic('%');
        self.counter +=1;
     }
     
     fn mulmod(&mut self){
        self.arithmetic('*');
        self.arithmetic('%');
        self.counter +=1;
     }
     
     fn exp(&mut self){
        if let Some(values) = self.get_n_elements(2){
            let max = BigUint::from(2u32).pow(256);
            let [base, expos] = values.try_into().unwrap();
            self.stack.push_front(Self::biguint_to_u256(base.modpow(&expos, &max)));
        }else{
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn signextend(&mut self){
        if let Some(values) = self.get_n_elements(2){
            let [byte_index, mut x] = values.try_into().unwrap();
            if byte_index >=BigUint::from(32u32){
                self.stack.push_front(Self::biguint_to_u256(x));
            } else {
                x = x & (BigUint::from(2u32).pow((8*(byte_index.to_usize().unwrap()+1)).try_into().unwrap())-BigUint::from(1u32));
                if ((x.clone() >> (8*byte_index.to_usize().unwrap() +7)) & BigUint::from(1u32)) == BigUint::from(1u32){
                    self.stack.push_front(Self::biguint_to_u256(Self::get_two_complement_of(x) | BigUint::from(2u32).pow(255)));
                } else {
                    self.stack.push_front(Self::biguint_to_u256(x));
                }
            }
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn comparison(&mut self, op : &str){
        if let Some(values) = self.get_n_elements(2){
            let [a,b] = values.try_into().unwrap();
            match op {
                "<" => {
                    if a < b {
                        self.stack.push_front(U256::one());
                    } else {
                        self.stack.push_front(U256::zero());
                    }
                }
                ">" => {
                    if a > b {
                        self.stack.push_front(U256::one());
                    } else {
                        self.stack.push_front(U256::zero());
                    }
                }
                "s<" => {
                    if ((a.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32)) ^ ((b.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32)) {
                        if a < b {
                            self.stack.push_front(U256::zero());
                        } else {
                            self.stack.push_front(U256::one());
                        }
                    } else {
                        if a < b {
                            self.stack.push_front(U256::one());
                        } else {
                            self.stack.push_front(U256::zero());
                        }
                    }
                }
                "s>" => {
                    if ((a.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32)) ^ ((b.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32)) {
                        if a > b {
                            self.stack.push_front(U256::zero());
                        } else {
                            self.stack.push_front(U256::one());
                        }
                    } else {
                        if a > b {
                            self.stack.push_front(U256::one());
                        } else {
                            self.stack.push_front(U256::zero());
                        }
                    }
                }
                "=" => {
                    if a == b {
                        self.stack.push_front(U256::one());
                    } else {
                        self.stack.push_front(U256::zero());
                    }
                }
                "=0" => {
                    self.stack.push_front(Self::biguint_to_u256(b));
                    if a == BigUint::from(0u32) {
                        self.stack.push_front(U256::one());
                    } else {
                        self.stack.push_front(U256::zero());
                    }
                }
                _ => println!("You thought this was a string match — but it was me, &str!"),
            }
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn iszero(&mut self){
        if let Some(values) = self.get_n_elements(1){
            let [a] = values.try_into().unwrap();
            if a == BigUint::from(0u32) {
                self.stack.push_front(U256::one());
            } else {
                self.stack.push_front(U256::zero());
            }
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn bitwise(&mut self, op : &str){
        if let Some(values) = self.get_n_elements(2){
            let [a, b] = values.try_into().unwrap();
            match op {
                "&" => self.stack.push_front(Self::biguint_to_u256(a & b)),
                "|" => self.stack.push_front(Self::biguint_to_u256(a | b)),
                "^" => self.stack.push_front(Self::biguint_to_u256(a ^ b)),
                "byte" => {
                    if a >=BigUint::from(32u32){
                        self.stack.push_front(U256::zero());
                    } else {
                        self.stack.push_front(Self::biguint_to_u256((b >> (8*(31-a.to_usize().unwrap()))) & BigUint::from(255u32)));
                    }
                }
                "<<" =>{
                    if a >= BigUint::from(256u32){
                        self.stack.push_front(U256::zero());
                    } else {
                        self.stack.push_front(Self::biguint_to_u256(b) << Self::biguint_to_u256(a));
                    }
                }
                ">>" => {
                    if a >= BigUint::from(256u32){
                        self.stack.push_front(U256::zero());
                    } else {
                        self.stack.push_front(Self::biguint_to_u256(b) >> Self::biguint_to_u256(a));
                    }
                }
                "sar" =>{
                    if (b.clone() & BigUint::from(2u32).pow(255))>>255 == BigUint::from(1u32){
                        if a >= BigUint::from(256u32) {
                            self.stack.push_front(U256::MAX);
                        } else {
                            self.stack.push_front((Self::biguint_to_u256(b) >> Self::biguint_to_u256(a.clone())) | ((U256::from(2).pow(Self::biguint_to_u256(a.clone())) - 1) << (U256::from(256) - Self::biguint_to_u256(a.clone()))));
                        }
                    } else {
                        if a >= BigUint::from(256u32){
                            self.stack.push_front(U256::zero());
                        } else {
                            self.stack.push_front(Self::biguint_to_u256(b) >> Self::biguint_to_u256(a));
                        }
                    }
                }
                _ => println!("mbrook hamza ziyada dyal lhawli - ba33333333333333333333333333333"),
            }
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn not(&mut self) {
        if let Some(values) = self.get_n_elements(1){
            let [a] = values.try_into().unwrap();
            self.stack.push_front(!Self::biguint_to_u256(a))
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn mstore(&mut self) {
        if let Some(values) = self.get_n_elements(2) {
            let [a, mut b] = values.try_into().unwrap();
            if (a.to_usize().unwrap()+31) >= self.memory.len().into() {
                self.memory.resize((((a.to_usize().unwrap()+31)/32)+1)*32,0);
            }
            match self.code[self.counter]{
                0x52 => {
                    for i in (0..32).rev() {
                        self.memory[a.to_usize().unwrap()+i]= (b.clone() & BigUint::from(255u32)).to_u8().unwrap();
                        b >>= 8;
                    }
                }
                0x53 => self.memory[a.to_usize().unwrap()]= (b.clone() & BigUint::from(255u32)).to_u8().unwrap(),
                _ => println!("bOOO, khle3tk?"),
            }
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn mload(&mut self) {
        if let Some(values) = self.get_n_elements(1){
            let [a] = values.try_into().unwrap();
            if (a.to_usize().unwrap()+31) >= self.memory.len().into() {
                self.memory.resize((((a.to_usize().unwrap()+31)/32)+1)*32,0);
            }
            let mut to_push: U256 = (U256::from(self.memory[a.to_usize().unwrap()]) << 8).into();
            for i in 1..32 {
                if i == 31 {
                    to_push = to_push | self.memory[a.to_usize().unwrap()+i].into();
                    break;
                }
                to_push = ( to_push | self.memory[a.to_usize().unwrap()+i].into() ) << 8;
            }
            self.stack.push_front(to_push);
        } else {
            println!("there is not enough value in stack");
        }
        self.counter +=1;
     }
     
     fn msize(&mut self) {
        self.stack.push_front(U256::from(self.memory.len()));
        self.counter +=1;
     }
     
     fn u256_to_biguint(value: U256) -> BigUint {
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);
        BigUint::from_bytes_be(&bytes)
    }
    
    fn biguint_to_u256(value: BigUint) -> U256 {
        let bytes = value.to_bytes_be();
        let mut padded = [0u8; 32];
        let start = 32 - bytes.len();
        padded[start..].copy_from_slice(&bytes);
        U256::from_big_endian(&padded)
    }
    
    fn get_n_elements(&mut self, n: usize) -> Option<Vec<BigUint>> {
        if self.stack.len() < n {
            return None;
        }
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            let raw = self.stack.pop_front()?;
            values.push(Self::u256_to_biguint(raw));
        }
        Some(values)
    }

    fn get_two_complement_of(a: BigUint) -> BigUint {
        BigUint::from(2u32).pow(255)-(a & (BigUint::from(2u32).pow(255)-BigUint::from(1u32)))
    }
}
