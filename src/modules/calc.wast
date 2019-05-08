(module
  (import "ns1" "add" (func $add (param i32 i32) (result i32)))
  (import "ns2" "mul" (func $mul (param i32 i32) (result i32)))

  (func (export "calc") (param i32 i32 i32) (result i32)
    (if (result i32)
      (i32.eq (i32.const 0) (get_local 0))
      (then
        (get_local 1)
        (get_local 2)
        (call $add))
      (else
        (get_local 1)
        (get_local 2)
        (call $mul)))))
