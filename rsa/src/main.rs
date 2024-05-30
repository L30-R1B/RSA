use std::time::{Duration, Instant};

mod script;
mod rsa;
mod primos;
mod aux;
mod compactar;

fn main() {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 1;

    for _ in 0..iterations {
        let start = Instant::now();
        
        let p = script::Processo::new("init");
        script::Processo::realiza_processo(p);

        let duration = start.elapsed();
        total_duration += duration;
    }

    let average_duration = total_duration / iterations as u32;
    println!("Tempo médio de execução: {:?}", average_duration);
}
