mod script;
mod rsa;
mod primos;
mod aux;
mod compactar;

use std::env;

fn main() {

    let arq_nome: Vec<String> = env::args().collect();

    let p = script::Processo::new(&arq_nome[1]);
    script::Processo::realiza_processo(p);
}
