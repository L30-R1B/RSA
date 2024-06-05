use num_bigint::{BigInt, BigUint, ToBigInt};
use once_cell::sync::Lazy;
use num_integer::Integer;
use crate::aux::{generate_random_biguint, mod_pow, DOIS, UM, ZERO};

static PRIMOS: Lazy<Vec<BigUint>> = Lazy::new(|| {
    vec![
        BigUint::from(2u8),
        BigUint::from(3u8),
        BigUint::from(5u8),
        BigUint::from(7u8),
        BigUint::from(11u8),
        BigUint::from(13u8),
        BigUint::from(17u8),
        BigUint::from(19u8),
        BigUint::from(23u8),
        BigUint::from(29u8),
        BigUint::from(31u8),
        BigUint::from(37u8),
        BigUint::from(41u8),
        BigUint::from(43u8),
        BigUint::from(47u8),
        BigUint::from(53u8),
        BigUint::from(59u8),
        BigUint::from(61u8),
        BigUint::from(67u8),
        BigUint::from(71u8),
        BigUint::from(73u8),
        BigUint::from(79u8),
        BigUint::from(83u8),
        BigUint::from(89u8),
        BigUint::from(97u8),
        BigUint::from(101u8),
        BigUint::from(103u8),
        BigUint::from(107u8),
        BigUint::from(109u8),
        BigUint::from(113u8),
        BigUint::from(107u8),
        BigUint::from(109u8),
        BigUint::from(113u8),
        BigUint::from(127u8),
        BigUint::from(131u8),
    ]
});

fn pequeno_teorema_de_fermat(p: BigUint, num_testes: usize, num_bits: usize) -> bool {
    if p <= (*UM).clone().into() {
        return false;
    }

    for _ in 0..num_testes {
        let mut a = generate_random_biguint(num_bits) + &*UM;
        while a >= p{
            a = generate_random_biguint(num_bits);
        }

        let resultado = a.modpow(&(p.clone() - &*UM), &p);

        if resultado != (*UM).clone().into() {
            return false;
        }
    }
    true
}

fn jacobi(a: &BigInt, n: &BigInt) -> i32 {
    if *n < (*UM).clone().into() {
        return 0;
    }

    let mut a = a.clone();
    let mut n = n.clone();
    let mut t = 1;

    while a != (*ZERO).clone().into() {
        while a.to_biguint().expect("") % &*DOIS == (*ZERO).clone().into() {
            a /= 2;
            let n_mod_8 = n.mod_floor(&8.to_bigint().unwrap());
            if n_mod_8 == 3.to_bigint().unwrap() || n_mod_8 == 5.to_bigint().unwrap() {
                t = -t;
            }
        }
        
        std::mem::swap(&mut a, &mut n);

        if a.clone() % 4 == 3.to_bigint().unwrap() && n.clone() % 4 == 3.to_bigint().unwrap() {
            t = -t;
        }
        
        a %= &n;
    }

    if n == (*UM).clone().into() {
        t
    } else {
        0
    }
}

fn solovay_strassen(n: &BigUint, k: usize, num_bits: usize) -> bool {
    if *n <= (*UM).clone().into() {
        return false;
    }
    if *n == (*DOIS).clone().into() || *n == BigUint::from(2u8) {
        return true;
    }
    if *n == (*ZERO).clone().into() {
        return false;
    }

    let n_minus_one = n - &*UM;
    let n_bigint = n.to_bigint().unwrap();
    
    for _ in 0..k {
        let mut a = generate_random_biguint(num_bits) + (&*DOIS);
        while a.clone() >= n.clone(){
            a = generate_random_biguint(num_bits) + (&*DOIS);
        }
        let a_bigint = a.to_bigint().unwrap();

        let x = jacobi(&a_bigint, &n_bigint);
        if x == 0 {
            return false;
        }

        let a_exp = a.modpow(&n_minus_one.div_floor(&BigUint::from(2u8)), n);
        let x_bigint = BigInt::from(x).mod_floor(&n.to_bigint().expect(""));

        if a_exp != x_bigint.to_biguint().unwrap() {
            return false;
        }
    }

    true
}

fn miller_rabin_test(n: &BigUint, k: u64, numero_bits: usize) -> bool {
    if n <= &*UM {
        return false;
    }

    let mut d = n - &*UM;
    let mut s = 0;
    while &d & &*UM == *ZERO {
        d >>= 1;
        s += 1;
    }

    for _ in 0..k {
        let a: BigUint = generate_random_biguint(numero_bits);
        let mut x = mod_pow(&a, &d, n);
        if x == *UM || x == n - &*UM {
            continue;
        }

        let mut pass = false;
        for _ in 0..s - 1 {
            x = (&x * &x) % n;
            if x == n - &*UM {
                pass = true;
                break;
            }
        }

        if !pass {
            return false;
        }
    }
    true
}



fn peneira_possiveis_primos(num:&BigUint)->bool{
    !PRIMOS.iter().any(|primo:&BigUint| num % primo == *ZERO)
}
fn primos_entre_si(mut a: BigUint, mut b: BigUint) -> bool {
    while b != *ZERO {
        let temp:BigUint = b.clone();
        b = a % &b;
        a = temp;
    }
    a == *UM
}
pub fn acha_coprimo(n: BigUint, num_bits: usize)->BigUint{
    loop{
        let m:BigUint = generate_random_biguint(num_bits / 2);
        if primos_entre_si(n.clone(), m.clone()){
            return m;
        }
    }
}
pub fn gera_provavel_primo(numero_bits:usize)->BigUint{
    loop{
        let n:BigUint = generate_random_biguint(numero_bits);
        if peneira_possiveis_primos(&n){
            if !pequeno_teorema_de_fermat(n.clone(), (numero_bits / 16) + 8, numero_bits/2){
                continue;
            }
            if !solovay_strassen(&n, (numero_bits / 32) + 8, numero_bits){
                continue;
            }
            if miller_rabin_test(&n, (numero_bits as u64 / 32) + 8, numero_bits){
                return n;
            }
            print!("z, ");
        }
    }
}
