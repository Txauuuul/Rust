use std::collections::HashMap;
use std::io::{self, Write};

// Calculadora
fn calculator(dict: &mut HashMap<String, i32>) {

    loop {
        print!("calc>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        let input = input.trim();
        if input == "bye" {
            break;
        }
        
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() != 3 {
            
        }

        let num1 = match tokens[0].parse::<f64>() {
            Ok(n) => n,
            Err(_) => match dict.get(tokens[0]) {
                Some(&val) => val as f64,
                None => {
                println!("{} no encontrado en el diccionario.", tokens[0]);
                continue;
                }
            }
        };

        let num2 = match tokens[2].parse::<f64>() {
            Ok(n) => n,
            Err(_) => match dict.get(tokens[2]) {
                Some(&val) => val as f64,
                None => {
                println!("{} no encontrado en el diccionario.", tokens[0]);
                continue;
                }
            }
        };

        match tokens[1] {
            "+" => println!("{}", num1 + num2),
            "-" => println!("{}", num1 - num2),
            "*" => println!("{}", num1 * num2),
            "/" => {
                if num2 != 0.0 {
                    println!("{}", num1 / num2);
                } else {
                    println!("No se puede dividir algo para cero, torpe.");
                }
            }
            _ => println!("Operación inválida. Intente de nuevo."),
        }
    }
}

// Diccionario
fn dictionary(dict: &mut HashMap<String, i32>) {

    loop {
        print!("process>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens[0] {
            "set" => {
                if tokens.len() != 3 {
                    println!("Comando inválido. Use: set <clave> <valor>");
                    continue;
                }
                let key = tokens[1].to_string();
                let value: i32 = match tokens[2].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Valor inválido.");
                        continue;
                    }
                };
                dict.insert(key, value);
            }
            "get" => {
                if tokens.len() != 2 {
                    println!("Comando inválido. Use: get <clave>");
                    continue;
                }
                let key = tokens[1].to_string();
                match dict.get(&key) {
                    Some(&value) => println!("{}", value),
                    None => println!("{}: no encontrado.", key),
                }
            }
            "list" => {
                for (key, value) in dict.iter() {
                    println!("{} = {}", key, value);
                }
            }
            "help" => {
                println!("Comandos :");
                println!("set <clave> <valor> - Establece el valor de una clave.");
                println!("get <clave> - Obtiene el valor de una clave.");
                println!("list - Muestra todas las claves y sus valores.");
                println!("bye - Salir del procesador de texto.");
            }
            "bye" => break,
            _ => println!("Comando no reconocido. Use 'help' para ver los comandos disponibles."),
        }
    }
}

fn main() {
    let mut shared_dict: HashMap<String, i32> = HashMap::new();

    loop {
        print!("sys>");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error al leer la entrada");

        match choice.trim() {
            "calc" => calculator(&mut shared_dict),
            "process" => dictionary(&mut shared_dict),
            "help" => {
                println!("Comandos disponibles:");
                println!("calc - Entrar a la calculadora.");
                println!("process - Entrar al procesador de texto.");
                println!("bye - Salir del programa.");
            }
            "bye" => {
                println!("Nos vemos.");
                break;
            }
            _ => println!("Comando no reconocido. Use 'help' para ver los comandos disponibles."),
        }
    }
}