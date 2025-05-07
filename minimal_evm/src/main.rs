mod evm_structur;
mod opcodes;
use crate::evm_structur::EVM;

fn hex_string_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|pair| {
            let s = std::str::from_utf8(pair).unwrap();
            u8::from_str_radix(s, 16).unwrap()
        })
        .collect() // [60,00,60,00,15]
}

fn main() {
    let hex_str = "60ff600052601160015360015159"; //push b push a
    let bytes = hex_string_to_bytes(hex_str);
    //println!("{:?}",bytes);
    
    
    let mut evm = EVM::new(bytes);
    //println!("{:#?}",evm);
    evm.run();
    println!("{:?}",evm);
    
}

