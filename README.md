# Module System of Rust 2018
Try to cover all possibilities of module dependencies in Rust 2018 to help people like me who is confusing about Rust's module system. Hopefully all dependency directions are covered, currently including looking at the followings, if you think something missing, please let me know:

- Parents (even looking at modules sitting at the root level)
- Siblings (even at root level)
- Cousins
- Sons



Cargo run outputs:

```rust
Caller = main::main(), Callee = d1/m1::prn()
Caller = d1/m1::prn(), Callee = d1/d11/m11::prn()
Caller = d1/d11/m11::prn(), Callee = d1/m1::prn2()
Caller = d1/d11/m11::prn(), Callee = util::prn()
Caller = main::main(), Callee = d1/m2::prn()
Caller = d1/m1_1::prn(), Callee = d1/m1::prn()
Caller = d1/m1::prn(), Callee = d1/d11/m11::prn()
Caller = d1/d11/m11::prn(), Callee = d1/m1::prn2()
Caller = d1/d11/m11::prn(), Callee = util::prn()
Caller = main::main(), Callee = d1/d11/m11::prn()
Caller = d1/d11/m11::prn(), Callee = d1/m1::prn2()
Caller = d1/d11/m11::prn(), Callee = util::prn()
Caller = main::main(), Callee = d2/m2::prn()
Caller = d2/m2::prn(), Callee = d1/d11/m11::prn()
Caller = d1/d11/m11::prn(), Callee = d1/m1::prn2()
Caller = d1/d11/m11::prn(), Callee = util::prn()
Caller = main::main(), Callee = d2/d22/m22::prn()
Caller = main::main(), Callee = config::prn()
Caller = config::prn(), Callee = d2/m2::prn()
Caller = d2/m2::prn(), Callee = d1/d11/m11::prn()
Caller = d1/d11/m11::prn(), Callee = d1/m1::prn2()
Caller = d1/d11/m11::prn(), Callee = util::prn()
Caller = config::prn(), Callee = d2/d22/m22::prn()
Caller = config::prn(), Callee = util::prn()
```

