(module
  (import "ns" "get" (func $wrapped_get (result i32)))
  (import "ns" "inc" (func $wrapped_inc))

  (func (export "wrapping_get") (result i32)
    (call $wrapped_get))

  (func (export "wrapping_inc")
    (call $wrapped_inc)))
