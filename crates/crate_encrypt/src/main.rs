use soft_aes::aes::{aes_enc_cbc, aes_dec_cbc};
use dotenvy::dotenv;
use std::env;
use hex::decode;
use hex::encode;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();


    let plaintext = b"Example plaintext.";

    let key_hex = env::var("ENC_KEY")?;
    let key = decode(key_hex)?;
    println!("Key: {:?}", key);
    
    let iv_hex = env::var("ENC_IV")?;
    let iv_bytes = decode(iv_hex)?;
    println!("ENC_IV: {:?}", iv_bytes);
    
    // Ensure the length is exactly 16 bytes
    if iv_bytes.len() != 16 {
        return Err("IV must be exactly 16 bytes".into());
    }
    
    // Convert to [u8; 16]
    let mut iv = [0u8; 16];
    iv.copy_from_slice(&iv_bytes);
    
    let padding = Some("PKCS7");
    
    let encrypted = aes_enc_cbc(plaintext, &key, &iv, padding).expect("Encryption failed");
    let decrypted = aes_dec_cbc(&encrypted, &key, &iv, padding).expect("Decryption failed");
    
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));
    println!("Expected: {:?}", String::from_utf8_lossy(plaintext));
    

    Ok(())

}