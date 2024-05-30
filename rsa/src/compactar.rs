use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use crate::aux::{ZERO, UM};

static BASE:&str = "!#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdefghijklmnopqrstuvwxyz{|}~";

pub fn compacta_biguint(num: &BigUint) -> String {
    let mut num_aux:BigUint = num.clone();
    let mut num_compac:String = String::new(); 

    let size:&BigUint = &BigUint::from(BASE.len() as u128);

    while num_aux > *ZERO {
        let c:char = BASE.chars().nth((num_aux.clone() % size).to_usize().unwrap()).expect("ERRO");
        num_compac.push(c);
        num_aux /= size;
    }

    if num_compac.is_empty() {
        num_compac.push(BASE.chars().nth(0).unwrap());
    }

    num_compac.chars().rev().collect()
}

pub fn descompacta_biguint(compactado: &str) -> BigUint {
    let mut num:BigUint = ZERO.clone();
    let mut base_power:BigUint = UM.clone();

    let size:BigUint = BigUint::from(BASE.len());

    for &c in compactado.as_bytes().iter().rev() {
        let index = BASE.find(c as char).expect("Caractere não encontrado na base");
        num += BigUint::from(index) * &base_power;
        base_power *= &size;
    }

    num
}

