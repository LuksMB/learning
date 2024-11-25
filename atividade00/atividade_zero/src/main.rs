use std::io;
use std::io::Write;

fn media_positivos(array: [i32; 10]) -> Option<i32> {
    let mut soma_positivos = 0;
    let mut num_positivos = 0;

    for item in array {
        if item > 0 {
            soma_positivos += item;
            num_positivos += 1;
        }
    }

    if num_positivos == 0 {
        None
    } else {
        Some(soma_positivos / num_positivos)
    }
}

fn produto_pares(array: [i32; 10]) -> i32 {
    let mut prod_pares = 1;

    for item in array {
        if item % 2 == 0 {
            prod_pares *= item;
        }
    }
    prod_pares
}

fn analisar_tupla(tupla: (i32, i32, i32)) -> (i32, i32, i32) {
    let mut maior = i32::MIN;
    let mut menor = i32::MAX;

    for &item in &[tupla.0, tupla.1, tupla.2] {
        if item > maior {
            maior = item;
        }
        if item < menor {
            menor = item;
        }
    }

    (tupla.0 + tupla.1 + tupla.2, maior, menor)
}

fn ler_tupla(tupla: (i32, i32, i32)) {

    print!("{{ {} ", tupla.0);

    for &item in &[ tupla.1, tupla.2] {
        print!("{} ", item);
    }

    print!("}}\n");
    io::stdout().flush().unwrap();
}

fn ler_inteiro() -> i32 {
    println!("Escolha um número: ");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Escolha um número válido!");

    match num.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 1,
    }
}

fn questao_01_main() {
    println!("\n Questão 1 \n");
    let numeros = [2, -3, 7, 0, 8, -1, 5, -4, 6, 10];
    match media_positivos(numeros) {
        Some(media) => println!("Média dos números positivos: {}", media),
        None => println!("Não há números positivos."),
    }
    let produto = produto_pares(numeros);
    println!("Produto dos números pares: {}", produto);
}

fn questao_02_main() {
    println!("\n Questão 2 \n");
    let a = ler_inteiro();
    let b = ler_inteiro();
    let c = ler_inteiro();
    let nova_tupla = (a, b, c);
    ler_tupla(analisar_tupla(nova_tupla));
}

fn main() {
    questao_02_main();
}
