use std::thread;

fn saudar(nome: String, vezes: i32) {
    for i in 0..vezes {
        println!("Olá, {}! (mensagem {})", nome, i + 1);
    }
}
fn main() {
    let nome = String::from("Maria");
    let vezes = 3;
    let thread = thread::spawn(move || {
        saudar(nome, vezes);
    });
    thread.join().unwrap();
}