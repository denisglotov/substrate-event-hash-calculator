Substrate event hash calculator
===============================

![Build status](https://github.com/denisglotov/substrate-event-hash-calculator/actions/workflows/rust.yml/badge.svg)

Substrate event is identified with the value from `Topic[0]`.

Calculate it with this tool as following: `cargo run Erc20::Transfer` - will give you the id of the `#[ink(event)]` 'Transfer' of the contract impl 'Erc20'.

To calculate the hash of `#[ink(topic)]` update [main.rs](src/main.rs) to

```rust
let hash = encoded_into_hash(&PrefixedValue {
    prefix: b"Erc20::Transfer::from",
    value: &expected_from,
}),
```

Based on the https://substrate.stackexchange.com/a/8376/4146 answer.
