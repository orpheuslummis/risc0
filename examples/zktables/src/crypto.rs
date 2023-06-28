// possibilities:
// - random seed
// - more flexible storage
// - config for parameters

use rand;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::LineEnding,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

use crate::{DEFAULT_NBITS, DEFAULT_PATH};

pub fn genkey(name: String) {
    let mut rng = rand::thread_rng();
    let bits = DEFAULT_NBITS;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let dir = std::path::Path::new(DEFAULT_PATH);
    std::fs::create_dir_all(dir).expect("failed to create directory");

    let path_pub = format!("{}pk_rsa_{}.pub", DEFAULT_PATH, name);
    let path_priv = format!("{}pk_rsa_{}.priv", DEFAULT_PATH, name);
    let public_key_bytes = pub_key.to_pkcs1_pem(LineEnding::default()).unwrap();
    let priv_key_bytes = priv_key.to_pkcs1_pem(LineEnding::LF).unwrap();
    std::fs::write(path_pub, public_key_bytes).expect("failed to write public key to file");
    std::fs::write(path_priv, priv_key_bytes).expect("failed to write secret key to file");
}

pub fn load_keys(name: String) -> (RsaPublicKey, RsaPrivateKey) {
    let path_pub = format!("{}pk_rsa_{}.pub", DEFAULT_PATH, name);
    let path_priv = format!("{}pk_rsa_{}.priv", DEFAULT_PATH, name);

    let pub_key_string = std::fs::read_to_string(path_pub).expect("failed to read public key");
    let priv_key_string = std::fs::read_to_string(path_priv).expect("failed to read private key");

    let pub_key = rsa::RsaPublicKey::from_pkcs1_pem(&pub_key_string).unwrap();
    let priv_key = rsa::RsaPrivateKey::from_pkcs1_pem(&priv_key_string).unwrap();
    (pub_key, priv_key)
}

pub fn encrypt(data: &[u8], pub_key: RsaPublicKey) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, data)
        .expect("failed to encrypt")
}

pub fn decrypt(data: &[u8], priv_key: RsaPrivateKey) -> Vec<u8> {
    priv_key
        .decrypt(Pkcs1v15Encrypt, data)
        .expect("failed to decrypt")
}
