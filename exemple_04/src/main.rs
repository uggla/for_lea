// Un exemple un peu plus réaliste....

// J'importe les définitions
use std::env;
use std::env::VarError;

fn is_it_serge() -> Result<String, String> {
    // Je cherche si la variable d'environnement SERGE existe. env::var retourne un
    // Result<String, VarError>  --> https://doc.rust-lang.org/std/env/fn.var.html
    //VarError est en fait un enum:
    //
    // pub enum VarError {
    //     NotPresent,
    //     NotUnicode(OsString),
    // }
    match env::var("SERGE") {
        //  Rust est stricte, il va nous obliger à checker tous les cas.
        Ok(env_var) => Ok(env_var.to_string()),
        //                                    ^- pas de ; je fais un return implicit
        Err(VarError::NotPresent) => {
            Err("Il n'y a pas de variable d'environnement Serge.".to_string())
        } // <-- pas de ; je fais un return implicite
        Err(VarError::NotUnicode(_)) => {
            //                   ^- j'ignore la possibilité de savoir de quel caractère on parle.
            Err("La variable contient un caractère non unicode.".to_string())
        } // <-- pas de ; je fais un return implicite
          //
          // On peut aussi faire un cas "générique" pour éviter les 2 cas d'erreurs
          // Err(_) => ....  mais toutes les combinaisons doivent être checké.
    }

    // On a en fait "transformé" le Result<String, VarError> en Result<String, String>
}

fn main() {
    let lapinou = is_it_serge();

    match lapinou {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }
}

// 🦉 uggla   master  …  rust  for_lea  exemple_04  cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target/debug/exemple_04`
// Il n'y a pas de variable d'environnement Serge.
// 🦉 uggla   master  …  rust  for_lea  exemple_04  SERGE="HELLO" cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target/debug/exemple_04`
// HELLO
