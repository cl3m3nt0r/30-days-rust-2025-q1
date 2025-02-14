# About Collect

To understand the collect function it's important to know that it is used to resolve an iterator. Recall that an iterator is a sequence of items that can be chained with other iterators. Iterators are lazy, which means they do not execute until you call code that consumes the iterator. Such code includes for loops and consuming adaptors. Consuming adaptors process all items in the iterator to produce some kind of output.

collect is one such consuming adaptor, which can be used to produce an instance of any type implementing the FromIterator trait. That trait is implemented for all of the collections in the standard library. Because FromIterator is implemented for many types, you need to let Rust know what sort of collection you desire. For example, if we want a Vec<char> from a string slice we could insert the type into our call to collect like so

```rust
let my_chars = "Hello, World!".chars().collect::<Vec<char>>();
// since the item type of the `String::chars()` iterator can be inferred to be `char`, we can omit that information
// let my_chars = "Hello, World!".chars().collect::<Vec<_>>();
```

The double colons and outer angle brackets (i.e. ::<_>) are known as the `turbofish`. In this case the `turbofish` is noisier syntax than we need. By specifying the type on the binding itself the collect can infer what collection it should create with less syntactical noise.

```rust
let my_chars: Vec<char> = "Hello, World!".chars().collect();
// or
// let my_chars: Vec<_> = "Hello, World!".chars().collect();
```
If we wanted a String with reversed characters we could do it like so

```rust
    let rev_chars: String = "Hello, World!".chars().rev().collect();
Rust's type inference is powerful enough that sometimes we don't need an explicit inline note:
```

```rust
fn get_reversed_chars(phrase: &str) -> Vec<char> {
    phrase.chars().rev().collect()
}
```

The return type of the get_reversed_chars function lets the collect function infer what collection to return. Here, though, we must specify the Vec<char> and not Vec<_>, as Rust doesn't perform type inference for function signatures. This is to ensure that someone reading the code can quickly know what types a function consumes and returns.
