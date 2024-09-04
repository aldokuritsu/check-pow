use sha2::{Sha256, Digest};

/// Fonction pour calculer le hash à partir du nonce, des données, et du timestamp
fn calculate_hash(nonce: u64, data: &str, timestamp: u64) -> String {
    let mut hasher = Sha256::new();
    
    // Utiliser le même ordre que dans la génération du hash : data, timestamp, nonce
    hasher.update(data);
    hasher.update(timestamp.to_string());
    hasher.update(nonce.to_string());

    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Fonction pour vérifier si un hash correspond au hash attendu
fn verify_pow(nonce: u64, data: &str, timestamp: u64, expected_hash: &str) -> bool {
    let computed_hash = calculate_hash(nonce, data, timestamp);
    if computed_hash == expected_hash {
        println!("PoW valide : {}", computed_hash);
        true
    } else {
        println!("La vérification du PoW a échoué.");
        println!("Expected: {}", expected_hash);
        println!("Computed: {}", computed_hash);
        false
    }
}

fn main() {
    // Paramètres d'entrée
    let nonce = 3361;
    let data = "toto";
    let timestamp = 1725376404;
    let expected_hash = "000118b7142f77b57ec7186fac7ba9371797a25e338a95004b6e53177d29a6fb";

    // Vérification de la preuve de travail
    verify_pow(nonce, data, timestamp, expected_hash);
}
