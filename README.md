🦀 Nothing special here!

### Inspired from

📌 [Rust Book](https://doc.rust-lang.org/book/)

📌 [Easy Rust](https://dhghomon.github.io/easy_rust/)

📌 [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)

#### Shortcuts

- [Variable](https://github.com/mhnaufal/rust-book/blob/main/src/variable.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

- [Data type](https://github.com/mhnaufal/rust-book/blob/main/src/data_types.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-02-data-types.html)

- [Function](https://github.com/mhnaufal/rust-book/blob/main/src/function.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

- [Control Flow](https://github.com/mhnaufal/rust-book/blob/main/src/control_flow.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

- [Ownership](https://github.com/mhnaufal/rust-book/blob/main/src/ownership.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

- [Borrowing](https://github.com/mhnaufal/rust-book/blob/main/src/borrowing.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

- [Struct](https://github.com/mhnaufal/rust-book/blob/main/src/struct.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

- [Enum](https://github.com/mhnaufal/rust-book/blob/main/src/enum.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

- [Vector](https://github.com/mhnaufal/rust-book/blob/main/src/vectors.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch08-01-vectors.html)

- [String](https://github.com/mhnaufal/rust-book/blob/main/src/strings.rs) 🔰 [Book](https://doc.rust-lang.org/book/https://doc.rust-lang.org/book/ch08-02-strings.html)

[HashMap](https://github.com/mhnaufal/rust-book/blob/main/src/maps.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

[Error](https://github.com/mhnaufal/rust-book/blob/main/src/errors.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

- Rust has two type of error, the recoverable one using Result<T, E> enum and unrecoverable using panic!
- When panic! happen, Rust will do _unwinding_ or walks back up the stack and cleans it

[Generic](https://github.com/mhnaufal/rust-book/blob/main/src/generic.rs) [Book](https://doc.rust-lang.org/stable/book/ch10-00-generics.html)

- Generics are abstract stand-ins for concrete types or other properties
-

```rust
fn largest<T>(list: &[T]) -> T {
```

The function _largest_ is generic over some type **T**. This function has one _parameter_ named _list_, which is a slice of values of type **T**. The largest function will _return_ a value of the same type **T**.

- Usually combined with Trait and Lifetime


#### Run

```rust
cargo run main.rs
```

#### Module tree

```rust
cargo modules generate tree
```

```rust
cargo modules generate tree --with-types
```
