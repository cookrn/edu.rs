TODO
====

Various todos...

## Rust for Rubyists

* Submit PR for `example2.rs` range type issue

```
example2.rs:4:14: 4:19 error: cannot determine a type for this bounded
type parameter: cannot determine the type of this integer; add a suffix
to specify the type explicitly

example2.rs:4   for num in range(0, 10) {
                             ^~~~~
```

* Submit PR for `example2.rs` unused variable warning

```rust
for num in range(0u, 10) {}
// vs
for _ in range(0u, 10) {}
```

* Submit PR for `example2.rb` not functioning as expected -- no prints?
* Submit PR for `example13.rs` range type error

* Submit PR for `example15.rs` `to_str` error

```
error: mismatched types: expected `&str` but found
`collections::string::String` (expected &-ptr but found struct
collections::string::String)
```

* Submit PR for `example16.rs` range type error (change to `1i`)

* Submit PR for `example16.rs` `format!` error

```
example16.rs:5:13: 5:34 error: mismatched types: expected `&str` but
found `collections::string::String` (expected &-ptr but found struct
collections::string::String)
example16.rs:5     println(format!("{:d}", num));
                           ^~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
```

* Submit PR for `example17.rs` range type error (change to `1i`)
* Submit PR for `example19.rs` for dead assignment
* Submit PR for `example22.rs` for weird str vs. String issues
* Submit PR for `example23.rs` for unused variable warning
* Submit PR for `example25.rs` for DuplexStream deprecation
* Submit PR for `example26.rs` "do is reserved word" error
