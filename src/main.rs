// En rust, le type Result sert a gérer les erreurs.
// Moi je vois ça comme une enveloppe que l'on retourne a l'appelant qui contient la réponse.
// A l'interieur il y a soit Ok(T) réponse positive soit Err(E) réponse négative. (T et E sont les
// types de ton choix)
//
// Si on regarde un peu sous le capot de result, un Result est en fait un enum.
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// Les enums en rust peuvent prendre des types (T et E).

// A noter que l'exemple ici est complètement bidon...
//
// ici                 T est une String -v,     v- E est une String (normalement un type d'erreur)
fn is_it_serge(lapin: &str) -> Result<String, String> {
    if lapin == "Serge" {
        // Tout se passe bien je renvoie un OK.
        Ok("🔞 aie! ouille ! Ah je suis bien !".to_string())
    } else {
        // Cas d'erreur je remonte une erreur.
        Err("🐰".to_string())
    }
}

fn main() {
    let lapinou = is_it_serge("Serge");
    //     ^-- lapinou est un Result. Donc il faut ouvrir l'enveloppe pour voir ce qu'il y a
    //     l'interieur

    // On a les possibilités un peu violente (souvent utilisé quand l'erreur ne peut pas être
    // récupéré)

    println!("{}", lapinou.unwrap());

    let lapinou2 = is_it_serge("Bugs Bunny");

    println!("{}", lapinou2.unwrap());
}
