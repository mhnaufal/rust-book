🦀 Nothing special here!

### Inspired from

📌 [Rust Book](https://doc.rust-lang.org/book/)

📌 [Easy Rust](https://dhghomon.github.io/easy_rust/)

📌 [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)

#### Shortcuts

1. [Variable](https://github.com/mhnaufal/rust-book/blob/main/src/variable.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

2. [Data type](https://github.com/mhnaufal/rust-book/blob/main/src/data_types.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-02-data-types.html)

3. [Function](https://github.com/mhnaufal/rust-book/blob/main/src/function.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

4. [Control Flow](https://github.com/mhnaufal/rust-book/blob/main/src/control_flow.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

5. [Ownership](https://github.com/mhnaufal/rust-book/blob/main/src/ownership.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

6. [Borrowing](https://github.com/mhnaufal/rust-book/blob/main/src/borrowing.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

7. [Struct](https://github.com/mhnaufal/rust-book/blob/main/src/struct.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

8. [Enum](https://github.com/mhnaufal/rust-book/blob/main/src/enum.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

9. [Vector](https://github.com/mhnaufal/rust-book/blob/main/src/vectors.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch08-01-vectors.html)

10. [String](https://github.com/mhnaufal/rust-book/blob/main/src/strings.rs) 🔰 [Book](https://doc.rust-lang.org/book/https://doc.rust-lang.org/book/ch08-02-strings.html)

11. [HashMap](https://github.com/mhnaufal/rust-book/blob/main/src/maps.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

12. [Error](https://github.com/mhnaufal/rust-book/blob/main/src/errors.rs) 🔰 [Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

- Rust has two type of error, the recoverable one using Result<T, E> enum and unrecoverable using panic!
- When panic! happen, Rust will do _unwinding_ or walks back up the stack and cleans it

13. [Generic](https://github.com/mhnaufal/rust-book/blob/main/src/generic.rs) 🔰 [Book](https://doc.rust-lang.org/stable/book/ch10-00-generics.html)

- Generics are abstract stand-ins for concrete types or other properties

```rust
fn largest<T>(list: &[T]) -> T {
```

- The function _largest_ is generic over some type **T**. This function has one _parameter_ named _list_, which is a slice of values of type **T**. The largest function will _return_ a value of the same type **T**.

- Usually combined with Trait and Lifetime

14. [Trait](https://github.com/mhnaufal/rust-book/blob/main/src/trait.rs) 🔰 [Book](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)

- **Trait** are similiar to **interface** in other programming language
- Declare a function/method/implementation name without giving the body function
- Trait can also has default implementation
- Function can also take type of trait for their parameter(s)

15. [Lifetime](https://github.com/mhnaufal/rust-book/blob/main/src/lifetime.rs) 🔰 [Book](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)

- Every refence has a _lifetime_
- Most of the time, lifetime are implicit, like type. However, we must annotate lifetime when lifetime references could be related in multiple ways.
- It's only about Scope
- Syntax => 'a, &'b

15. [Test](https://github.com/mhnaufal/rust-book/blob/main/src/tests.rs) 🔰 [Book](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)

- Rust provide built-in functionallity for testing, whether unit testing or even integration testing
- Keywords used, **assert**, **assert_eq**, **assert_ne**

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
