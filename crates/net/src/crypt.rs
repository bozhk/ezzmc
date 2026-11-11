//! Шифрование online-mode: AES-128/CFB8 поверх потока и RSA-обмен ключом.
// VANILLA: Crypt#encryptUsingKey, CipherEncoder/CipherDecoder (AES/CFB8/NoPadding, key = iv = secret)
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::Aes128;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

/// Одна сторона CFB8 (у шифратора и дешифратора свой регистр сдвига).
pub struct Cfb8 {
    aes: Aes128,
    reg: [u8; 16],
}

impl Cfb8 {
    pub fn new(secret: &[u8; 16]) -> Self {
        Self {
            aes: Aes128::new(GenericArray::from_slice(secret)),
            reg: *secret,
        }
    }

    fn keystream(&self) -> u8 {
        let mut b = GenericArray::clone_from_slice(&self.reg);
        self.aes.encrypt_block(&mut b);
        b[0]
    }

    fn shift_in(&mut self, c: u8) {
        self.reg.copy_within(1.., 0);
        self.reg[15] = c;
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        for b in data {
            let c = *b ^ self.keystream();
            self.shift_in(c);
            *b = c;
        }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        for b in data {
            let c = *b;
            *b = c ^ self.keystream();
            self.shift_in(c);
        }
    }
}

/// Пара шифров соединения.
pub struct CipherPair {
    pub enc: Cfb8,
    pub dec: Cfb8,
}

impl CipherPair {
    pub fn new(secret: &[u8; 16]) -> Self {
        Self {
            enc: Cfb8::new(secret),
            dec: Cfb8::new(secret),
        }
    }
}

/// RSA/PKCS1v15 публичным ключом сервера (X.509 DER).
pub fn rsa_encrypt(public_key_der: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let key = RsaPublicKey::from_public_key_der(public_key_der)
        .map_err(|e| format!("ключ сервера: {e}"))?;
    key.encrypt(&mut rand_core::OsRng, Pkcs1v15Encrypt, data)
        .map_err(|e| format!("RSA: {e}"))
}

pub fn random_secret() -> [u8; 16] {
    use rand_core::RngCore;
    let mut s = [0u8; 16];
    rand_core::OsRng.fill_bytes(&mut s);
    s
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_chunking() {
        let k = [7u8; 16];
        let plain: Vec<u8> = (0..1000u32).map(|i| (i * 31) as u8).collect();
        let mut a = plain.clone();
        let mut e = Cfb8::new(&k);
        e.encrypt(&mut a[..13]);
        e.encrypt(&mut a[13..]);
        assert_ne!(a, plain);
        let mut d = Cfb8::new(&k);
        d.decrypt(&mut a[..500]);
        d.decrypt(&mut a[500..]);
        assert_eq!(a, plain);
    }
}
