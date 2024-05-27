use std::collections::HashMap;
use std::io::{self, Write};

// Calculadora
fn calculator() {

    loop {
        println!("calc>");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        let input = input.trim();
        if input == "bye" {
            break;
        }
        
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() != 3 {
            
        }

        let num1: f64 = match tokens[0].parse() {
            Ok(n) => n,
            Err(_) => {
                continue;
            }
        };
        let num2: f64 = match tokens[2].parse() {
            Ok(n) => n,
            Err(_) => {
                continue;
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
                    println!("");
                }
            }
            _ => println!("Operación inválida. Intente de nuevo."),
        }
    }
}

// Diccionario
fn dictionary() {
    let mut dictionary: HashMap<String, i32> = HashMap::new(); // Creamos un diccionario vacío para almacenar los datos.

    loop {
        print!("process>");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada"); //El usuario nos introduce el texto.

        let tokens: Vec<&str> = input.trim().split_whitespace().collect(); //Separamos las claves de los valores.

        match tokens[0] {
            "set" => {
                if tokens.len() != 3 {
                    println!("Comando inválido"); // Si el comando set no tiene el formato adecuado, mostramos un mensaje de error.
                }
                let key = tokens[1].to_string();
                let value: i32 = tokens[2].parse().expect("Valor inválido");
                dictionary.insert(key, value); // Insertamos la clave y el valor en el diccionario.
            }
            "get" => {
                if tokens.len() != 2 {
                    println!("Error."); // En caso de que usemos mal get, nos saltará un error.
            
                }
                let key = tokens[1].to_string();
                match dictionary.get(&key) {
                    Some(&value) => println!("{}.", value), //Si la clave existe, sabremos su valor.
                    None => println!("{}: not found.", key), //Si la clave no existe, nos dirá que no ha sido encontrada.
                }
            }
            "list" => {
                if dictionary.is_empty() {
                    println!(""); // En caso de que el diccionario esté vacío, no devolvemos nada.
                } else {
                    for (key, value) in &dictionary {
                        println!("{} = {}.", key, value); //Buscamos valores en el diccionario.
                    }
                }
            }
            "help" => {
            println!("            set> Le damos un valor a una clave. Por ejemplo, set hola 5. A partir de ahora hola será igual a 5.
            get> Le damos una clave y el programa nos devolverá el valor. Por ejemplo, get hola, el programa nos devolverá 5.
            list> El programa nos devuelve todas las claves con sus respectivos valores almacenados en el diccionario.
            bye> Nos salimos del programa.");
            }
            
            "bye" => break,
            _ => {
                let key = tokens[0].to_string();
                match dictionary.get(&key) {
                    Some(&value) => println!("{} = {}.", key, value), // Hace lo mismo que el get, solo que sin esta opción, no he encontrado manera de obtener el resultado de abajo.
                    None => println!("{}: not found.", key), // En caso de no tener la palabra guardadam nos dirá "not found".
                }
            }
        }
    }
}

fn main() {
    loop {
        print!("mysys>");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error.");

        match choice.trim() {
            "calc" => calculator(),
            "process" => dictionary(),
            "help" => {
                println!(" calc> Entrar a la calculadora.
       process> Entrar al procesador de texto.
       bye> Salir del programa.");
            }
            "bye" => {
                println!("Nos vemos.");
                break;
            }
            _ => println!("Error."),
        }
    }
}