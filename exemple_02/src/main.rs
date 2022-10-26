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

    // On a la 2ème possibilité un peu moins violente (souvent utilisé quand l'erreur ne peut pas être
    // récupéré)
    // On remplace unwrap() par expect("msg") qui nous en dit un peu plus sur le sujet.

    println!("{}", lapinou.expect("Erreur ce n'est pas Serge"));
    //                 ^- ici pas d'erreur on récupère T (String) et on l'affiche.

    let lapinou2 = is_it_serge("Bugs Bunny");

    println!("{}", lapinou2.expect("Erreur ce n'est pas Serge"));
    //                        ^-- ici on a une erreur, le expect() nous fait partir en panique !
    //                        mais avec plus d'info -v
    // thread 'main' panicked at 'Erreur ce n'est pas Serge: "Erreur c'est juste un 🐰"', src/main.rs:23:29
    //
    // on est d'accord ca change pas grand chose.
}
