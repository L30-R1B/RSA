mod script;
mod rsa;
mod primos;
mod aux;
mod compactar;

fn main() {
    let p = script::Processo::new("init");
    script::Processo::realiza_processo(p);
}
