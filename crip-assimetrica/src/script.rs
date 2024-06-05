use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use num_bigint::BigUint;
use std::path::Path;

use crate::compactar::{compacta_biguint, descompacta_biguint};
use crate::rsa::{ChavePub, ChavePriv, Chaves};

pub struct Instrucao{
    comando: String,
    num_parametros: usize,
}
pub struct Processo{
    instrucoes:Vec<Instrucao>,
}

fn ler_arquivo(caminho_arq: &str) -> io::Result<Vec<String>> {
    if Path::new(caminho_arq).exists() {
        let file = File::open(caminho_arq)?;
        let reader = BufReader::new(file);
        let mut texto: Vec<String> = Vec::new();

        for linha in reader.lines() {
            texto.push(linha?);
        }

        Ok(texto)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Arq inexistente ou não pertencente ao caminho especificado ! ! !"))
    }
}
fn escreve_txt_arq(arq_nome: &str, txt: Vec<String>) -> bool {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(arq_nome);

    match file {
        Ok(mut file) => {
            for char_cod in txt {
                if write!(file, "{} ", char_cod).is_err() {
                    return false;
                }
            }
            if write!(file, "\n").is_err() {
                return false;
            }
            true
        }
        Err(_) => false,
    }
}

fn criptografar_arq_txt(arq_caminho_pub: &str, arq_caminho_ascii: &str, arq_caminho_cript: &str, manter_arq_original: char) -> bool {
    let msg_ascii_texto = match ler_arquivo(arq_caminho_ascii) {
        Ok(content) => content,
        Err(_) => return false,
    };
    if manter_arq_original == 'n'{
        let _ = fs::remove_file(arq_caminho_ascii);
    }
    let c_pub_arq_texto = match ler_arquivo(arq_caminho_pub) {
        Ok(content) => content,
        Err(_) => return false,
    };
    let n:BigUint = descompacta_biguint(&c_pub_arq_texto[0]);
    let e:BigUint = descompacta_biguint(&c_pub_arq_texto[1]);
    
    let c_pub = ChavePub { n, e };
    for msg in msg_ascii_texto{
        let crip = Chaves::criptografa_msg(msg.into_bytes(), &c_pub);
        escreve_txt_arq(arq_caminho_cript, crip);
    }
    true
}
fn descriptografar_arq_txt(arq_caminho_priv: &str, arq_caminho_cript: &str, arq_caminho_dcript: &str, manter_arq_original: char)->bool{
    let msg_cript_texto = match ler_arquivo(arq_caminho_cript) {
        Ok(content) => content,
        Err(_) => return false,
    };
    if manter_arq_original == 'n'{
        let _ = fs::remove_file(arq_caminho_cript);
    }
    let c_priv_arq_texto = match ler_arquivo(arq_caminho_priv) {
        Ok(content) => content,
        Err(_) => return false,
    };
    let n:BigUint = descompacta_biguint(&c_priv_arq_texto[0]);
    let d:BigUint = descompacta_biguint(&c_priv_arq_texto[1]);

    let c_priv:ChavePriv = ChavePriv{n, d};

    for linha in msg_cript_texto{
        let msg_cod: Vec<String> = linha.split_whitespace()
                                    .map(|s| s.to_string())
                                    .collect();
        
        let msg_decod:Vec<u8> = Chaves::descriptografa_msg(msg_cod, &c_priv);

        escreve_txt_arq(arq_caminho_dcript, vec![(String::from_utf8_lossy(&msg_decod).to_string()).to_string()]);
    }
    return true;
}
fn salvar_chaves_arq(caminho:&str, id: u64, c:Chaves) -> io::Result<()> {
    let mut file;
    if caminho == "/"{
        file = File::create(format!("cPub{}.txt", id))?;
    }else{
        file = File::create(format!("{}cPub{}.txt", caminho, id))?;
    }
    writeln!(file, "{}\n{}", compacta_biguint(&c.c_pub.n), compacta_biguint(&c.c_pub.e))?;

    if caminho == "/"{
        file = File::create(format!("cPriv{}.txt", id))?;
    }else{
        file = File::create(format!("{}cPriv{}.txt", caminho, id))?;
    }

    writeln!(file, "{}\n{}", compacta_biguint(&c.c_priv.n), compacta_biguint(&c.c_priv.d))?;

    Ok(())
}
fn apagar_arquivo_chave(id:u64, caminho:&str) {
    if caminho != "/" {
        let _ = fs::remove_file(format!("{}cPub{}.txt", caminho, id));
        let _ = fs::remove_file(format!("{}cPriv{}.txt", caminho, id));
    }else{
        let _ = fs::remove_file(format!("cPub{}.txt", id));
        let _ = fs::remove_file(format!("cPriv{}.txt", id));
    }
}

impl Instrucao{
    fn new(comando:String)->Instrucao{
        let num_parametros = comando.split_whitespace().count();
        Instrucao{
            comando, 
            num_parametros,
        }
    }
    fn realiza_instrucao(self)->bool{
        let partes: Vec<&str> = self.comando.split(" ").collect();
        if partes[0] == "GEN_CHAVE" {
            if self.num_parametros > 3 {
                let num_bits:usize = partes[1].parse::<usize>().unwrap();
                let id: u64 = partes[2].parse::<u64>().unwrap();
                let c:Chaves = Chaves::new(num_bits);

                let _ = salvar_chaves_arq(partes[3], id, c);
            }
        }else if partes[0] == "REM_CHAVE"{
            if self.num_parametros > 2 {
                let id:u64 = partes[1].parse::<u64>().unwrap();
                apagar_arquivo_chave(id, partes[2]);
            }
        }else if partes[0] == "CRIPT_MSG"{
            if self.num_parametros > 4{
                criptografar_arq_txt(partes[1], partes[2], partes[3],  partes[4].chars().nth(0).unwrap());
            }
        }else if partes[0] == "DCRIP_MSG"{
            if self.num_parametros > 4{
                descriptografar_arq_txt(partes[1], partes[2], partes[3], partes[4].chars().nth(0).unwrap());
            }
        }else if partes[0] == "REM_ARQ"{
            if self.num_parametros > 1 {
                let _ = fs::remove_file(partes[1]);
            }
            return false;
        }
        return true;
    }
}
impl Processo{
    pub fn new(arq_caminho: &str) -> Processo {
        let mut instrucoes:Vec<Instrucao> = Vec::new();

        match ler_arquivo(&arq_caminho) {
            Ok(texto) => {
                for linha in texto {
                    instrucoes.push(Instrucao::new(linha));
                }
            }
            Err(_) => println!("ERRO"),
        }

        return Processo{
            instrucoes,
        }
    }
    pub fn realiza_processo(p:Processo)->bool{
        for instru in p.instrucoes{
            Instrucao::realiza_instrucao(instru);
        }
        return true;
    }
}

