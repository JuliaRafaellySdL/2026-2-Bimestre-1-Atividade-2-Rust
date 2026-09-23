---

# Relatório de implementação de linha de execução em Rust

## Introdução

Este relato faz parte do processo avaliativo da disciplina de Sistemas Operacionais no curso superior em análise e desenvolvimento de sistemas, ofertado na diretoria acadêmica de gestão e tecnologia da informação no campus natal-central do Instituto Federal de Educação, Ciência e Tecnologia do Rio Grande do Norte.

Tem como objetivo principal relatar como implementar linhas de execução na linguagem Rust.

O grupo de trabalho foi formado por Julia Rafaelly Siqueira de Lima, Lídia Rebeka da Silva Fernandes e Lyonara da Silva Carmêlo.

---

## Implementando múltiplas linhas de execução em Rust

### Informações gerais sobre Rust

Rust é uma linguagem focada em segurança de memoria, alto desempenho e concorrência segura (Fearless Concurrency). Seu objetivo principal é fornecer infraestrutura de sistema segura sem a necessidade de um *Garbage Collector*. Está disponível oficialmente através do site oficial  `rust-lang.org` e do gerenciador de pacote e compilação `cargo`.

Qual o objetivo e o paradgima da linguagem? o Rust foca em segurança de memória e infraestrutura de rede. Ela é multiparadigma pois permite o uso de diferentes formas de programação como: Imperativa, estruturada, funcional e orientada a objeto. 
Esta disponível onde? Windows, Linux, macOS, servidores, entre outros.

---

### Criando linhas de execução

Em Rust, as threads nativas do sistema operacional são criadas utilizando a função `std::thread::spawn`. Cada thread recebe uma *closure* (função anônima) que contém o código a ser executado em segundo plano.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread secundária: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    handle.join().unwrap(); // Aguarda a finalização da thread
}

```

---

### Passando valores para linhas de execução

Para transferir a posse (*ownership*) de variáveis para dentro de uma thread, utiliza-se a palavra-chave `move` antes da *closure*. Isso garante que a thread seja dona dos dados e evita disputas de memória (*data races*).

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vetor dentro da thread: {:?}", v);
    });

    handle.join().unwrap();
}

```

---

### Múltiplas linhas de execução

É possível criar várias threads iterando em um laço de repetição e armazenando os seletores (*handles*) em um vetor para aguardar o término de todas com o método `join()`.

```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Executando thread número {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

```

---

### Considerações finais

A implementação de threads em Rust destaca-se pela segurança em tempo de compilação. O modelo de posse (*ownership*) e tipos como `Arc` e `Mutex` evitam *data races* antes mesmo do código ser executado, tornando a programação concorrente mais previsível e eficiente quando comparada a outras linguagens.

Obs: os códigos em rust equivalentes aos códigos em python apresentados em aula podem ser encontrados em: 2026-2-Bimestre-1-Atividade-2-Rust>src>rust

---