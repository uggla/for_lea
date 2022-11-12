fn is_it_serge(lapin: &str) -> Result<String, String> {
    if lapin == "Serge" {
        // Tout se passe bien je renvoie un OK.
        Ok("🔞 aie! ouille ! Ah je suis bien !".to_string())
    } else {
        // Cas d'erreur je remonte une erreur.
        Err("Erreur c'est juste un 🐰".to_string())
    }
}

fn main() {
    let lapinou = is_it_serge("Serge");

    // On a la 3ème possibilité qui est de tester le retour. Souvent ce que l'on veut faire pour
    // prévenir l'utilisateur.
    // Pour faire cela souvent on utilise le pattern matching. (une sorte de super switch/case).

    match lapinou {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }

    let lapinou2 = is_it_serge("Bugs Bunny");

    match lapinou2 {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{} {}", e, "normal on est pas dans le métro boulet !"),
    }

    // Pas de panique on a géré l'erreur avec un beau message pour l'utilisateur.
}

// 🦉 uggla   master  …  rust  for_lea  exemple_03  cargo run
//    Compiling exemple_03 v0.1.0 (/home/uggla/workspace/rust/for_lea/exemple_03)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.41s
//      Running `target/debug/exemple_03`
// 🔞 aie! ouille ! Ah je suis bien !
// Erreur c'est juste un 🐰 normal on est pas dans le métro boulet !
