// possibilities:
// - random seed
// - more flexible storage
// - config for parameters

use rand::{self, rngs::OsRng};
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::LineEnding,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use serde::{ser::Error, Serialize, Serializer};

pub const DEFAULT_NBITS: usize = 2048;

use crate::Vote;

#[derive(Clone, Debug)]
pub struct Pubkey(pub RsaPublicKey);
#[derive(Clone, Debug)]
pub struct Privkey(pub RsaPrivateKey);

impl Serialize for Pubkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pub_key_pem = self
            .0
            .to_pkcs1_pem(LineEnding::default())
            .map_err(Error::custom)?;

        serializer.serialize_str(&pub_key_pem)
    }
}

impl Serialize for Privkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let priv_key_pem = self
            .0
            .to_pkcs1_pem(LineEnding::default())
            .map_err(Error::custom)?;

        serializer.serialize_str(&priv_key_pem)
    }
}

pub fn generate_keypair() -> (Pubkey, Privkey) {
    let mut rng = OsRng;
    let bits = DEFAULT_NBITS;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    (Pubkey(pub_key), Privkey(priv_key))
}

pub fn write_keys_to_dir(name: String, keydir: String, pubkey: Pubkey, privkey: Privkey) {
    let path_pub = format!("{}pk_rsa_{}.pub", keydir, name);
    let path_priv = format!("{}pk_rsa_{}.priv", keydir, name);

    if std::path::Path::new(&path_pub).exists() || std::path::Path::new(&path_priv).exists() {
        panic!("keys already exist in filesystem");
    }

    std::fs::create_dir_all(keydir).expect("failed to create directory");

    let pub_key_bytes = pubkey.0.to_pkcs1_pem(LineEnding::default()).unwrap();
    let priv_key_bytes = privkey.0.to_pkcs1_pem(LineEnding::default()).unwrap();

    std::fs::write(path_pub, pub_key_bytes).expect("failed to write public key to file");
    std::fs::write(path_priv, priv_key_bytes).expect("failed to write secret key to file");
}

pub fn load_keys(name: String, dir: String) -> (Pubkey, Privkey) {
    let path_pub = format!("{}pk_rsa_{}.pub", dir, name);
    let path_priv = format!("{}pk_rsa_{}.priv", dir, name);

    let pubkey_string = std::fs::read_to_string(path_pub).expect("failed to read public key");
    let privkey_string = std::fs::read_to_string(path_priv).expect("failed to read private key");

    let pubkey = rsa::RsaPublicKey::from_pkcs1_pem(&pubkey_string).unwrap();
    let privkey = rsa::RsaPrivateKey::from_pkcs1_pem(&privkey_string).unwrap();
    (Pubkey(pubkey), Privkey(privkey))
}

pub fn encrypt(data: &Vote, pub_key: RsaPublicKey) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let data = &[*data];
    pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, data)
        .expect("failed to encrypt")
}

pub fn decrypt(data: &Vote, priv_key: RsaPrivateKey) -> Vec<u8> {
    let data = &[*data];
    priv_key
        .decrypt(Pkcs1v15Encrypt, data)
        .expect("failed to decrypt")
}
