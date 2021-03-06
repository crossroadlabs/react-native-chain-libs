use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use js_chain_libs::{UtxoPointer, Account, Input, Inputs, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputFromUtxo(
  env: JNIEnv, _: JObject, utxo_pointer: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let utxo_pointer = utxo_pointer.rptr(&env)?;
    utxo_pointer
      .typed_ref::<UtxoPointer>()
      .map(|utxo_ptr| Input::from_utxo(utxo_ptr))
      .and_then(|input| input.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputFromAccount(
  env: JNIEnv, _: JObject, account: JRPtr, v: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let account = account.rptr(&env)?;
    let v = v.rptr(&env)?;
    account
      .typed_ref::<Account>()
      .zip(v.typed_ref::<Value>())
      .map(|(account, v)| Input::from_account(account, v))
      .and_then(|input| input.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputValue(
  env: JNIEnv, _: JObject, input: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let input = input.rptr(&env)?;
    input.typed_ref::<Input>().and_then(|input| input.value().rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputsNew(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| Inputs::new().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputsSize(
  env: JNIEnv, _: JObject, inputs: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let inputs = inputs.rptr(&env)?;
    inputs
      .typed_ref::<Inputs>()
      .map(|inputs| inputs.size())
      .and_then(|size| size.into_jlong().jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputsGet(
  env: JNIEnv, _: JObject, inputs: JRPtr, index: jlong
) -> jobject {
  handle_exception_result(|| {
    let inputs = inputs.rptr(&env)?;
    inputs
      .typed_ref::<Inputs>()
      .map(|inputs| inputs.get(usize::from_jlong(index)))
      .and_then(|input| input.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputsAdd(
  env: JNIEnv, _: JObject, inputs: JRPtr, item: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let inputs = inputs.rptr(&env)?;
    inputs
      .typed_ref::<Inputs>()
      .zip(item.owned::<Input>(&env))
      .map(|(inputs, item)| inputs.add(item))
  })
  .map(|_| JObject::null())
  .jresult(&env)
}
