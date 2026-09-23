use std::thread;
use std::time::{Duration, Instant};

fn trabalhador(numero: i32, tempo_trabalho: u64) {
    println!("Trabalhador {} começou", numero);
    thread::sleep(Duration::from_secs(tempo_trabalho));
    println!(
        "Trabalhador {} terminou (levou {}s)",
        numero, tempo_trabalho
    );
}
fn main() {
    println!("Iniciando 5 trabalhadores...");
    let inicio = Instant::now();
    let mut threads = Vec::new();
    for i in 0..5 {
        let thread = thread::spawn(move || {
            trabalhador(i, 2);
        });

        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let tempo_total = inicio.elapsed();
    println!("\nTodos os trabalhadores terminaram!");
    println!("Tempo total: {:.2}s", tempo_total.as_secs_f64());
    println!("(Se fosse sequencial, levaria ~10s)");
}