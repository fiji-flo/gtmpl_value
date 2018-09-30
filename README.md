# gtmpl_value &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]
[Build Status]: https://travis-ci.org/fiji-flo/gtmpl_value.svg?branch=master
[travis]: https://travis-ci.org/fiji-flo/gtmpl_value
[Latest Version]: https://img.shields.io/crates/v/gtmpl_value.svg
[crates.io]: https://crates.io/crates/gtmpl_value


**The internal value type for [gmtpl-rust][gtmpl_value-github]**

---

```toml
[dependencies]
gtmpl_value = "0.3.3"
```

* [gtmpl_value at crates.io](https://crates.io/crate/gtmpl_value)
* [gtmpl_value documentation](https://docs.rs/crate/gtmpl_value)

## Current State

`gtmpl_value` is a basic implementation for internal values within
[`gtmpl-rust`][gtmpl_value-github]. It is used as to represent values parsed from
the template and from the context.


## Usage

The [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) trait is
implemented for:

* `String, &str`
* most numeric types `u64, u32, …, i64, i32, …, f64, f32`
* `bool`
* `Vec<Value>, &[Value]`
* `HashMap<String, Value>`

[`gtmpl_derive`](https://github.com/fiji-flo/gtmpl_derive) provides a custom
`derive` for structs.

```rust
extern crate gtmpl_value;
use gtmpl_value::Value;

fn main() {
    let v: Value = "something".into();
    println!("{}", v);
}
```

[gtmpl_value-github]: https://github.com/fiji-flo/gtmpl-rust
