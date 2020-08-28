
# JVMTI Bindings for Rust

This project provides complete JVMTI bindings for Rust 

## Warning

The project is currently only available for testing.

## Example
```
cd example
make // TODO
```

## Thanks

- rvmti [https://github.com/rel-eng/rvmti](https://github.com/rel-eng/rvmti)
- jni-rs [https://github.com/jni-rs/jni-rs](https://github.com/jni-rs/jni-rs)
- rust-jvmti [https://github.com/xea/rust-jvmti](https://github.com/xea/rust-jvmti)

## TODO

- [x] Simplify JVMTIEnv functions, learn & use design for jni-rs#descriptors 
- [ ] Options: parse & design option struct
- [ ] Check all functions to ensure that the memory is released correctly [jvmti docs](https://docs.oracle.com/javase/8/docs/platform/jvmti/jvmti.html#Deallocate) (keyword: The array should be freed with Deallocate.)
- [ ] Rich test cases

## License

 * Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
