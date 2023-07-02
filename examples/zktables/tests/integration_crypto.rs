use zktables_core::crypto::{decrypt, encrypt, keygen, load_keys};

#[test]
fn crypto_cycle() {
    // keygen("test".to_string());
    // let (pubkey, privkey) = load_keys("test".to_string());
    // let data = b"Hello, world!";
    // let enc = encrypt(data, pubkey);
    // let dec = decrypt(&enc, privkey);
    // assert_eq!(data, dec.as_slice());
}
