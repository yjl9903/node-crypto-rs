#![deny(clippy::all)]

use napi::{bindgen_prelude::*, CallContext, JsBuffer, JsObject};

mod aes_gcm;

use aes_gcm::AesGcmTask;

#[macro_use]
extern crate napi_derive;

#[js_function(2)]
fn encrypt_aes_gcm(ctx: CallContext) -> Result<JsObject> {
  let key = ctx.get::<JsBuffer>(0)?.into_ref()?;
  let plain = ctx.get::<JsBuffer>(1)?.into_ref()?;
  let task = AesGcmTask::new_encrypt(key, plain);
  ctx
    .env
    .spawn(task)
    .map(|async_task| async_task.promise_object())
}

#[js_function(2)]
fn decrypt_aes_gcm(ctx: CallContext) -> Result<JsObject> {
  let key = ctx.get::<JsBuffer>(0)?.into_ref()?;
  let encrypted = ctx.get::<JsBuffer>(1)?.into_ref()?;
  let task = AesGcmTask::new_decrypt(key, encrypted);
  ctx
    .env
    .spawn(task)
    .map(|async_task| async_task.promise_object())
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("encrypt_aes_gcm", encrypt_aes_gcm)?;
  exports.create_named_method("decrypt_aes_gcm", decrypt_aes_gcm)?;

  Ok(())
}
