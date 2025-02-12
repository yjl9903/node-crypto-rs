#![deny(clippy::all)]

use napi::{bindgen_prelude::*, CallContext, JsBuffer, JsObject};

mod aes_gcm;

use aes_gcm::AesGcmTask;

#[macro_use]
extern crate napi_derive;

// #[napi]
// pub fn sum(a: i32, b: i32) -> i32 {
//   a + b
// }

// #[module_exports]
// fn init(mut exports: JsObject) -> Result<()> {
//   exports.create_named_method("encryptAesGcm", encrypt_aes_gcm)?;

//   Ok(())
// }

#[js_function(2)]
#[napi(
  ts_args_type = "buffer: Buffer, key: Buffer",
  ts_return_type = "Promise<Buffer>"
)]
fn encrypt_aes_gcm(ctx: CallContext) -> Result<JsObject> {
  let buffer = ctx.get::<JsBuffer>(0)?.into_ref()?;
  let key = ctx.get::<JsBuffer>(1)?.into_ref()?;
  let task = AesGcmTask::new(buffer, key);
  ctx
    .env
    .spawn(task)
    .map(|async_task| async_task.promise_object())
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("encrypt_aes_gcm", encrypt_aes_gcm)?;

  Ok(())
}
