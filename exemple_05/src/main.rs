use std::env;
use std::env::VarError;

fn is_it_serge() -> Result<String, VarError> {
    // Bon souvent tu veux pas faire la gestion d'erreur ici mais dans l'appelant.
    // C'est la ou le '?'' vas être utile il va faire remonter l'erreur (bubble).
    let env_var = env::var("SERGE")?;
    // Si il y a une erreur on la remonte, sinon on continue...

    // la fn fait des trucs...

    Ok(env_var) // return implicit
}

fn main() {
    let lapinou = is_it_serge();

    match lapinou {
        Ok(msg) => println!("{}", msg),
        Err(VarError::NotPresent) => println!("Il n'y a pas de variable d'environnement Serge."),
        Err(_) => println!("Erreur avec la variable d'environnement..."),
    };
}

// 🦉 uggla   master  …  rust  for_lea  exemple_05  cargo run
//    Compiling exemple_05 v0.1.0 (/home/uggla/workspace/rust/for_lea/exemple_05)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.55s
//      Running `target/debug/exemple_05`
// Il n'y a pas de variable d'environnement Serge.
// 🦉 uggla   master  …  rust  for_lea  exemple_05  SERGE="HELLO" cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target/debug/exemple_05`
// HELLO
