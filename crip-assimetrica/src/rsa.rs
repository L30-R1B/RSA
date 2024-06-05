use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;
use crate::compactar::{compacta_biguint, descompacta_biguint};
use crate::primos;
use crate::aux::{UM, calcular_inverso_multiplicativo, mod_pow};

pub struct ChavePub{
    pub n:BigUint,
    pub e:BigUint,
}

pub struct ChavePriv{
    pub n:BigUint,
    pub d:BigUint,
}

impl ChavePub{}
impl ChavePriv{}

pub struct Chaves{
    pub c_priv: ChavePriv,
    pub c_pub: ChavePub,
} 

impl Chaves{
    pub fn new(numero_bits:usize)->Chaves{
        let mut c_pub:ChavePub = ChavePub{n: BigUint::from(1u8), e: BigUint::from(1u8)};
        let mut c_priv:ChavePriv = ChavePriv{n: BigUint::from(1u8), d: BigUint::from(1u8)};
        
        let mut inverso_multiplicativo:BigInt = (*UM).clone().into();

        let tam_primos = numero_bits / (2 as usize);

        let mut p:BigUint = primos::gera_provavel_primo(tam_primos);
        let mut q:BigUint = UM.clone().into();

        let mut estado:bool = false;

        while inverso_multiplicativo == (*UM).clone().into(){   
            if !estado{
                q = primos::gera_provavel_primo(tam_primos);
                estado = true;
            }else{
                p = primos::gera_provavel_primo(tam_primos);
                estado = false;
            }

            c_pub.n = &p * &q;
            c_pub.e = primos::acha_coprimo(c_pub.n.clone(), numero_bits);

            c_priv.n = c_pub.n.clone();

            let prod_p_q_inclementado:BigUint = (&p - &*UM) * (&q - &*UM);
            
            inverso_multiplicativo = calcular_inverso_multiplicativo(&c_pub.e.clone().into(), &prod_p_q_inclementado.into());
        }
        c_priv.d = inverso_multiplicativo.to_biguint().expect("Conversão falhou");

        return Chaves{c_priv, c_pub}; 
    }

    fn criptografa_char(caractere:u8, c_pub:&ChavePub)->String{
        return compacta_biguint(&mod_pow(&BigUint::from(caractere), &c_pub.e, &c_pub.n));
    }

    pub fn criptografa_msg(mensagem:Vec<u8>, c_pub:&ChavePub)->Vec<String>{
        let mut msg_codificada:Vec<String> = Vec::new();
        for caractere in mensagem{
            msg_codificada.push(Self::criptografa_char(caractere,&c_pub));
        }
        return msg_codificada;
    }
    
    fn descriptografa_char(caractere_codificado:BigUint, c_priv:&ChavePriv)->u8{
        return (&mod_pow(&caractere_codificado, &c_priv.d, &c_priv.n)).to_u8().expect("Valor fora do intervalo de u8");
    }
    pub fn descriptografa_msg(mensagem_codificada:Vec<String>, c_priv:&ChavePriv)->Vec<u8>{
        let mut msg:Vec<u8> = Vec::new();
        for caractere_codificado in mensagem_codificada{
            msg.push(Self::descriptografa_char( descompacta_biguint(&caractere_codificado) , &c_priv));
        }

        return msg;
    }
}





