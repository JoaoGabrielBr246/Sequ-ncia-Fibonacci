use std::io;

fn main() {
    println!("Digite dois números: ");
    
    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin().read_line(&mut num1)
        .expect("Erro ao ler a linha");

    io::stdin().read_line(&mut num2)
        .expect("Erro ao ler a linha");

    let num1: u8 = num1.trim().parse()
        .expect("Digite um número válido");
    let num2: u8 = num2.trim().parse()
        .expect("Digite um número válido");

    fibonacci(num1, num2);
}

fn fibonacci(mut n1: u8, mut n2: u8) {
    println!("{}", n1);
    println!("{}", n2);

    let mut result;

    for _ in 0..=253 {
        result = n1 + n2;
        println!("{}", result);
        n1 = n2;
        n2 = result;
    }
}
