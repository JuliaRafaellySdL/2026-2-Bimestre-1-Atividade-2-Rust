use std::thread;
use std::time::Duration;

fn minha_funcao() {
    println!("Thread iniciada!");
    thread::sleep(Duration::from_secs(2));
    println!("Thread finalizada!");
}
fn main() {
    let thread = thread::spawn(|| {
        minha_funcao();
    });
    thread.join().unwrap();
    println!("Programa principal finalizado!");
}