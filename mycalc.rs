use std::collections::HashMap;
use std::io;

fn main() {
    let mut variables: HashMap<String, f32> = HashMap::new();

    loop {
        //De esta manera, aparecerá siempre "mycalc" cuando la calculadora nos conteste.
        print!("mycalc> ");
        //Parte en la que se le solicitan datos al usuario.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");

        // Sirve para eliminar espacios en blanco, sin esto (aun que parezca inservible, no funciona el código).
        let input = input.trim();

        // Cerrar el programa en caso de que digamos Bye o bye.
        if input == "bye" || input == "Bye" {
            break;
        }

        // En caso de que se ponga el símbolo de igualación, el programa lo detectará y asimilará que la parte izquierda y la derecha del signo, tienen el mismo valor numérico.
        if input.contains('=') {
            let parts: Vec<&str> = input.split('=').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let var_name = parts[0].to_string();
                let valor: f32 = match parts[1].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        continue;
                    }
                };
                variables.insert(var_name.clone(), valor);
            } else {
            }
        // En caso de que se ponga el símbolo de suma, el programa entenderá que estamos realizando una suma, dividiendo las dos partes de esta suma como parte 1 y parte 2. En caso de 5+3, el 5 será la parte 1 (al estar antes de la suma) y el 3 será parte 2(al estar después de la suma).
        } else if input.contains('+') {
            let parts: Vec<&str> = input.split('+').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let num1 = match variables.get(parts[0]) {
                    Some(&n) => n,
                    None => {
                        match parts[0].parse() {
                            Ok(n) => n,
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                };
                let num2 = match variables.get(parts[1]) {
                    Some(&n) => n,
                    None => {
                        match parts[1].parse() {
                            Ok(n) => n,
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                };
                //Si el "if" de bye ni el de igualación, no han sido satisfechos, pasaremos al if de la suma, por lo que, si introducimos los datos correctamente, nos enseñará el resultado.
                let resultado = num1 + num2;
                println!("{}", resultado);
            } else {
                println!("Error: Formato de suma incorrecto.");
            }
        //En caso de que ninguno de los "ifs" anteriores se haya cumplido, y hayamos puesto solo un número, con el cual no realizaremos ninguna operación, simplemente nos aparecerá el númnero. En caso de que pongamos carácteres entre comillas, estos aparecerán, pero sin comillas. En caso de que pongamos carácteres sin comillas, nos dará un error, donde no reconoce estos carácteres.
        } else if input.starts_with('"') && input.ends_with('"') {
            println!("{}", input.trim_matches('"'));
        } else if let Ok(num) = input.parse::<f32>() {
            println!("{}", num);
        } else {
            println!("Not know: {}", input.trim_matches('"'));
        }
    }
}