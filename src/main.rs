use chrono::{DateTime, Local};
use ring::{rand, signature::{self, EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_ASN1_SIGNING}};
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize)]
struct Transaction {
    from: String,
    to: String,
    time: DateTime<Local>,
    amount: f64,
    signature: String
}


fn main() {
    let rng = rand::SystemRandom::new();
    // this code generates private key with sha-256 cryptography and secp256r1 elliptical curve algorithm
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &rng).unwrap(); 
    // this code generates public key and private key as key pair (means private key is hidden in the public key and related to each other) from pkcs8_bytes type of algorithm
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    // the message itself
    let msg: &[u8] = b"Kj is the Best of World";
    // this code signs the message with private key and generates the signature
    let sign = key_pair.sign(&rng, msg);
    let signature_str = match sign {
    Ok(signature) => {
        let encoded = base64::encode(signature.as_ref());
        println!("Signature: {}", encoded);
        encoded
    },
    Err(e) => {
        println!("Error signing: {}", e);
        return;
    }
};
    // Here we just get the public key key directly from the key pair.
    let peer_public_keys_bytes = key_pair.public_key().as_ref();
    println!("\npublic key as bytes: {:?}", peer_public_keys_bytes);
    // this code verifies if the public key and the signature match each other
    let peer_public_keys = signature::UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_ASN1, peer_public_keys_bytes);
    println!("\npublic key Self {:?}", peer_public_keys);

    let mut get_dest_public_key = String::new();
    println!("Enter your Destination Public key : ");
    io::stdin().read_line(&mut get_dest_public_key).expect("failed to read public key line");
    println!("Enter your amount of transaction : ");
    let mut get_tr_amount_raw: String = String::new();
    io::stdin().read_line(&mut get_tr_amount_raw).expect("failed to read amount transaction line");
    let amount = match get_tr_amount_raw.trim().parse::<f64>() {
        Ok(val) => Some(val),
        _ => {
            println!("an error occurred!");
            None
        }
    };
    println!("\namount : {:?}", amount);
    println!("\nget destination public key : {:?}", get_dest_public_key);
    
    let tr: Transaction = Transaction {
        from: base64::encode(pkcs8_bytes.as_ref()), 
        to: get_dest_public_key.to_string(), 
        time: Local::now(), 
        amount: amount.unwrap(), 
        signature: signature_str, 
    };

     let mut get_transactions: Vec<Transaction> = if let Ok(content) = fs::read_to_string("transaction.json") {
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
     } else {
        Vec::new()
     };
     get_transactions.push(tr);
     let stringifying = serde_json::to_string(&get_transactions).unwrap();
     let _ = fs::write("transaction.json", stringifying);
}


