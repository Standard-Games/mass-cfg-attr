Mass Cfg Attr
=============

This custom derive macro allows you to wrap multiple attributes in the same `#[cfg_attr(..., ...)]`

For example, you might be using another custom derive to tag methods for additional wiring, but in the test runtime, 
that additional wiring won't work. To work around this you'd have to use `cfg_attr` over and over again. This introduces
extra work, makes your code harder to read and creates extra risk. `mass_cfg_attr` helps mitigate that:

**Without mass_cfg_attr**:
```rust
struct MyStruct;

#[derive(SomeAutoWirer)]
impl MyStruct {
    #[cfg_attr(not(any(test, doctest)), wire(options))]
    fn func_one(self) {
        // ...
    }

    #[cfg_attr(not(any(test, test)), wire(options))] // mistake the compiler won't see
    fn func_two(self) {
        // ...
    }

    #[cfg_attr(not(any(test, doctest)), wire(options))]
    fn func_three(self) {
        // ...
    }
}
```

**With mass_cfg_attr**

```rust
struct MyStruct;

#[mass_cfg_attr(not(any(test, doctest)), [wire, wire_more])]
#[derive(SomeAutoWirer)]
impl MyStruct {
    #[wire(options)]
    fn func_one(self) {
        // ...
    }

    #[wire(options)]
    fn func_two(self) {
        // ...
    }

    #[wire_more(options)]
    fn func_three(self) {
        // ...
    }
}
```
