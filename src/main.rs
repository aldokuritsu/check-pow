use sha2::{Sha256, Digest};

// Type alias pour plus de clarté
type HashString = String;

/// Fonction pour calculer le hash à partir du nonce, des données et du timestamp
/// Renvoie une `HashString`.
fn calculate_hash(nonce: u64, data: &str, timestamp: u64) -> HashString {
    let mut hasher = Sha256::new();
    
    // Mise à jour du hasher avec les éléments dans l'ordre : data, timestamp, nonce
    hasher.update(data);
    hasher.update(timestamp.to_string());
    hasher.update(nonce.to_string());

    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Fonction pour vérifier la preuve de travail (PoW)
/// Renvoie `Result<bool, String>` pour indiquer si la vérification a réussi ou échoué avec un message d'erreur.
fn verify_pow(nonce: u64, data: &str, timestamp: u64, expected_hash: &str) -> Result<bool, String> {
    let computed_hash = calculate_hash(nonce, data, timestamp);
    
    if computed_hash == expected_hash {
        Ok(true) // PoW est valide
    } else {
        // Retourne une erreur avec un message détaillé
        Err(format!(
            "La vérification du PoW a échoué.\nExpected: {}\nComputed: {}",
            expected_hash, computed_hash
        ))
    }
}


fn main() {
    // Paramètres d'entrée
    let nonce: u64 = 3361;
    let data = "toto";
    let timestamp: u64 = 1725376404;
    let expected_hash = "000118b7142f77b57ec7186fac7ba9371797a25e338a95004b6e53177d29a6fb";

    // Vérification de la preuve de travail (PoW)
    match verify_pow(nonce, data, timestamp, expected_hash) {
        Ok(_) => println!("PoW valide !"),
        Err(error) => eprintln!("{}", error), // Affichage de l'erreur
    }
}
