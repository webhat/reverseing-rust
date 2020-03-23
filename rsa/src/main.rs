extern crate rsa;
extern crate rand;

use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use rand::rngs::OsRng;

fn crypt() {
    let mut rng = OsRng;
    let bits = 2048;
    let key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    let data = b"hello world";

    // Encrypt
    let enc_data = key.encrypt(&mut rng, PaddingScheme::PKCS1v15, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);


    // Decrypt
    let dec_data = key.decrypt(PaddingScheme::PKCS1v15, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);

    println!("{:?}",dec_data);
}


fn main() {
    crypt();
}
