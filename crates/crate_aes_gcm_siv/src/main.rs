use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    aead::Error,
    Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
};

fn main()->Result<(),Error > {
 
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())?;
    println!("ciphertext: {:?}", String::from_utf8_lossy(&ciphertext));

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;
    println!("plaintext: {:?}", String::from_utf8_lossy(&plaintext));

    assert_eq!(&plaintext, b"plaintext message");

    Ok(())
}

