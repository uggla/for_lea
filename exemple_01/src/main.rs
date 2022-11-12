// En rust, le type Result sert à gérer les erreurs.
// Moi je vois ça comme une enveloppe que l'on retourne a l'appelant qui contient la réponse.
// A l'intérieur il y a soit Ok(T) pas d'erreur le résultat et de type T, soit Err(E), on a une erreur de type E. (T et E sont les
// types de ton choix)
//
// Si on regarde un peu sous le capot d'un Result, un Result est en fait un enum,
// avec 2 variants Ok(T) et Err(E).
//
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// Les enums en rust peuvent prendre des types (T et E).

// A noter que l'exemple ici est complètement bidon...
//
// ici                 T est une String -v,     v- E est une String
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
    //     ^-- lapinou est un Result. Donc il faut ouvrir l'enveloppe pour voir ce qu'il y a
    //     l'intérieur

    // On a la 1ere possibilité un peu violente (souvent utilisé quand l'erreur ne peut pas être
    // récupéré)

    println!("{}", lapinou.unwrap());
    //                 ^- ici pas d'erreur on récupère T (String) et on l'affiche.

    let lapinou2 = is_it_serge("Bugs Bunny");

    println!("{}", lapinou2.unwrap());
    //                        ^-- ici on a une erreur, le unwrap() nous fait partir en panique !
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "Erreur c'est juste un 🐰"', src/main.rs:41:29
}
