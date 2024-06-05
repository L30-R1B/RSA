use num_bigint::{BigInt, BigUint};
use rand::{rngs::ThreadRng, Rng};
use once_cell::sync::Lazy;

pub static ZERO: Lazy<BigUint> = Lazy::new(|| BigUint::from(0u8));
pub static UM: Lazy<BigUint> = Lazy::new(|| BigUint::from(1u8));
pub static DOIS: Lazy<BigUint> = Lazy::new(|| BigUint::from(2u8));

pub fn calcular_inverso_multiplicativo(e: &BigInt, n: &BigInt) -> BigInt {
    let (gcd, x, _) = alg_euclides_extendido(e, n);
    if gcd != UM.clone().into() {
        return UM.clone().into(); 
    }

    let result = (x % n + n) % n.clone();

    return result;
}
pub fn alg_euclides_extendido(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b == &ZERO.clone().into() {
        return (a.clone(), UM.clone().into(), ZERO.clone().into());
    }

    let (d, x, y) = alg_euclides_extendido(b, &(a % b));
    let new_x = y.clone();
    let new_y = x - (a / b) * y;

    (d, new_x, new_y)
}
pub fn generate_random_biguint(bits: usize) -> BigUint {
    let mut rng:ThreadRng = rand::thread_rng();

    let mut random_bytes:Vec<u8> = Vec::with_capacity((bits + 7) / 8);
    for _ in 0..((bits + 7) / 8) {
        random_bytes.push(rng.gen::<u8>());
    }
    BigUint::from_bytes_be(&random_bytes)
}

pub fn mod_pow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = UM.clone();
    let mut expo = exponent.clone();
    let mut num = base.clone() % modulus;

    while expo > *ZERO {
        if &expo & &*UM == *UM {
            result *= &num;
            result %= modulus;
        }
        expo >>= 1;
        num = (&num * &num) % modulus;
    }

    result
} 
