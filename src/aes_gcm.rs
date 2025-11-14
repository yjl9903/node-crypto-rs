use napi::bindgen_prelude::*;
use aes_gcm::{
  aead::{Aead, OsRng},
  AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};

pub struct EncryptAesGcm {
  key: Buffer,
  data: Buffer,
}

pub struct DecryptAesGcm {
  key: Buffer,
  data: Buffer,
}

impl EncryptAesGcm {
  pub fn new(key: Buffer, data: Buffer) -> Self {
    Self { key, data }
  }
}

impl DecryptAesGcm {
  pub fn new(key: Buffer, data: Buffer) -> Self {
    Self { key, data }
  }
}

#[napi]
impl Task for EncryptAesGcm {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let key = self.key.as_ref();
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let data = self.data.as_ref();
    match cipher.encrypt(&nonce, data) {
      Ok(encrypted) => {
        let mut output = Vec::new();
        output.extend_from_slice(&nonce);
        output.extend_from_slice(&encrypted);
        Ok(output)
      }
      Err(e) => Err(Error::from_reason(e.to_string())),
    }
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }

  fn finally(self, _: Env) -> Result<()> {
    drop(self.key);
    drop(self.data);
    Ok(())
  }
}

#[napi]
impl Task for DecryptAesGcm {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let key = self.key.as_ref();
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(&key);
    let data = self.data.as_ref();
    let (nonce, data) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    match cipher.decrypt(nonce, data.as_ref()) {
      Ok(plain) => Ok(plain),
      Err(e) => Err(Error::from_reason(e.to_string())),
    }
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }

  fn finally(self, _: Env) -> Result<()> {
    drop(self.key);
    drop(self.data);
    Ok(())
  }
}
