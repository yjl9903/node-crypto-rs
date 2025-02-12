use aes_gcm::{
  aead::{Aead, OsRng},
  AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use napi::{Env, Error, JsBuffer, JsBufferValue, Ref, Result, Task};

#[derive(Clone, Copy)]
enum TaskMode {
  Encrypt,
  Decrypt,
}

pub struct AesGcmTask {
  mode: TaskMode,
  key: Ref<JsBufferValue>,
  data: Ref<JsBufferValue>,
}

impl AesGcmTask {
  pub fn new_encrypt(key: Ref<JsBufferValue>, data: Ref<JsBufferValue>) -> Self {
    AesGcmTask {
      mode: TaskMode::Encrypt,
      key,
      data,
    }
  }

  pub fn new_decrypt(key: Ref<JsBufferValue>, data: Ref<JsBufferValue>) -> Self {
    AesGcmTask {
      mode: TaskMode::Decrypt,
      key,
      data,
    }
  }
}

impl Task for AesGcmTask {
  type Output = Vec<u8>;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let data = self.data.as_ref().as_ref();

    let key = self.key.as_ref().as_ref();
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(&key);

    match self.mode {
      TaskMode::Encrypt => {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

        match cipher.encrypt(&nonce, data) {
          Ok(enc) => {
            let mut output = Vec::new();
            output.extend_from_slice(&nonce);
            output.extend_from_slice(&enc);
            Ok(output)
          }
          Err(e) => Err(Error::from_reason(e.to_string())),
        }
      }
      TaskMode::Decrypt => {
        let (nonce, data) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce);

        match cipher.decrypt(nonce, data.as_ref()) {
          Ok(plain) => Ok(plain),
          Err(e) => Err(Error::from_reason(e.to_string())),
        }
      }
    }
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(env.create_buffer_with_data(output)?.into_raw())
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    self.key.unref(env)?;
    self.data.unref(env)?;
    Ok(())
  }
}
