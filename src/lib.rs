#![allow(unused)]

extern crate wabt;
extern crate wasmer_runtime;

#[macro_use]
use wasmer_runtime::{func, Func, imports, instantiate, Ctx, Value, Instance};

#[cfg(test)]
mod tests {
    use super::*;

    fn subtract(_: &mut Ctx, a: i32, b: i32) -> i32 {
        a - b
    }

    macro_rules! load_wasm {
        ($file:expr) => {{
            let wasm = wabt::wat2wasm(include_bytes!($file).as_ref()).unwrap();
            wasm
        }};
    }

    #[test]
    fn test_multiple_stateless_wasm_modules() {
        let add_wasm = load_wasm!("modules/add.wast");
        let mul_wasm = load_wasm!("modules/mul.wast");
        let calc_wasm = load_wasm!("modules/calc.wast");

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

    fn inc_and_get(instance: &Instance) -> i32 {
        let get_func = instance.dyn_func("get").unwrap();
        let inc_func = instance.dyn_func("inc").unwrap();

        inc_func.call(&[]).unwrap();

        let res = get_func.call(&[]).unwrap();

        assert_eq!(1, res.len());

        if let Value::I32(val) = res[0] {
            val
        } else {
            panic!()
        }
    }

    fn wrapping_inc_and_get(instance: &Instance) -> i32 {
        let get_func = instance.dyn_func("wrapping_get").unwrap();
        let inc_func = instance.dyn_func("wrapping_inc").unwrap();

        inc_func.call(&[]).unwrap();

        let res = get_func.call(&[]).unwrap();

        assert_eq!(1, res.len());

        if let Value::I32(val) = res[0] {
            val
        } else {
            panic!()
        }
    }

    #[test]
    fn test_two_isolated_stateful_instances_of_the_same_wasm_module() {
        let wasm = load_wasm!("modules/counter.wast");
        let instance1 = instantiate(&wasm, &imports! {}).unwrap();
        let instance2 = instantiate(&wasm, &imports! {}).unwrap();

        let res = inc_and_get(&instance1);
        assert_eq!(1, res);

        let res = inc_and_get(&instance1);
        assert_eq!(2, res);

        let res = inc_and_get(&instance2);
        assert_eq!(1, res);
    }

    #[test]
    fn test_shared_stateful_wasm_instance() {
        let wrapped_wasm = load_wasm!("modules/counter.wast");
        let wrapped_instance = instantiate(&wrapped_wasm, &imports! {}).unwrap();

        let res = inc_and_get(&wrapped_instance);
        assert_eq!(1, res);

        let res = inc_and_get(&wrapped_instance);
        assert_eq!(2, res);

        let wasm = load_wasm!("modules/wrapping_counter.wast");
        let import_object = imports! {
            "ns" => wrapped_instance,
        };

        let instance = instantiate(&wasm, &import_object).unwrap();

        let res = wrapping_inc_and_get(&instance);
        assert_eq!(3, res);
    }
}
