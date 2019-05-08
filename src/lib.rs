#![allow(unused)]

extern crate wabt;
extern crate wasmer_runtime;

#[macro_use]
use wasmer_runtime::{func, imports, instantiate, Ctx, Value};

#[cfg(test)]
mod tests {
    use super::*;

    fn subtract(_: &mut Ctx, a: i32, b: i32) -> i32 {
        a - b
    }

    #[test]
    fn test_multiple_wasm_modules() {
        let add_wasm = wabt::wat2wasm(include_bytes!("modules/add.wast").as_ref()).unwrap();
        let mul_wasm = wabt::wat2wasm(include_bytes!("modules/mul.wast").as_ref()).unwrap();
        let calc_wasm = wabt::wat2wasm(include_bytes!("modules/calc.wast").as_ref()).unwrap();

        let add_instance = instantiate(&add_wasm, &imports! {}).unwrap();
        let mul_instance = instantiate(&mul_wasm, &imports! {}).unwrap();

        let import_object = imports! {
            "ns1" => add_instance,
            "ns2" => mul_instance,
            "ns3" => {
                "sub" => func!(subtract),
            },
        };

        let calc_instance = instantiate(&calc_wasm, &import_object).unwrap();

        let func = calc_instance.dyn_func("calc").unwrap();

        let add_res = func
            .call(&[Value::I32(0), Value::I32(10), Value::I32(20)])
            .unwrap();

        let mul_res = func
            .call(&[Value::I32(1), Value::I32(10), Value::I32(20)])
            .unwrap();

        let sub_res = func
            .call(&[Value::I32(2), Value::I32(30), Value::I32(10)])
            .unwrap();

        assert_eq!(vec![Value::I32(30)], add_res);
        assert_eq!(vec![Value::I32(200)], mul_res);
        assert_eq!(vec![Value::I32(20)], sub_res);
    }
}
