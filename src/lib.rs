#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod aes_gcm;

use aes_gcm::{EncryptAesGcm, DecryptAesGcm};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn encrypt_aes_gcm(key: Buffer, data: Buffer) -> AsyncTask<EncryptAesGcm> {
  AsyncTask::new(EncryptAesGcm::new(key, data))
}

#[napi]
pub fn decrypt_aes_gcm(key: Buffer, data: Buffer) -> AsyncTask<DecryptAesGcm> {
  AsyncTask::new(DecryptAesGcm::new(key, data))
}
