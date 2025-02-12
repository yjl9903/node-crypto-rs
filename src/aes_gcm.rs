use aes_gcm::{
  aead::{Aead, OsRng},
  AeadCore, Aes256Gcm, AesGcm, Key, KeyInit,
};
use napi::{Env, JsBuffer, JsBufferValue, Ref, Result, Task};

pub struct AesGcmTask {
  buffer: Ref<JsBufferValue>,
  key: Ref<JsBufferValue>,
}

impl AesGcmTask {
  pub fn new(buffer: Ref<JsBufferValue>, key: Ref<JsBufferValue>) -> Self {
    AesGcmTask { buffer, key }
  }
}

impl Task for AesGcmTask {
  type Output = Vec<u8>;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let buffer = self.buffer.as_ref().as_ref();

    let key = self.key.as_ref().as_ref();
    let key: &Key<Aes256Gcm> = key.into();

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

    match cipher.encrypt(&nonce, buffer) {
      Ok(enc) => Ok(enc),
      Err(_) => Ok(vec![]),
    }
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(env.create_buffer_with_data(output)?.into_raw())
  }
}
