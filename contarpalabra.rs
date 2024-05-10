fn main() {
    let s: &str = "Hola";
    let letra = s.chars().next().unwrap_or(' ');

            println!("En la palabra '{}' la primera letra es '{}'.", s, letra);

        
            //La función chars lo que hace es almacenar los carácteres de la palabra Hola, la función next hace que obtenga el programa obtenga
            //la letra H, ya que si no, como me explicaste el otro día, el valor sería 0, y por último unwrap_or, lo que hace es que aun que no haya 
            //valor en S, no de error de compilacion. Otra opcion sería obviar el unwrap_or y utilizar la funcion match con su respectivo some y none. 
            //Necesitamos hacer esto ya que una función char no puede aparecer en println por si sola, al parecer."

}
