# Async operations in RUST :crab:

- [YT-1: The Why, What, and How of Futruees and Async/Await in Rust ](https://www.youtube.com/watch?v=9_3krAQtD2k&t=325s)
- [YT-2: The Why, What, and How of Pinning in Rust](https://www.youtube.com/watch?v=DkMwYxfSYNQ)



# Table of Contents
- [Crate Links](#crate-links)
- [Links](#links)
- [Futures](#futures)
  * [Java](#java)
  * [Javascript](#javascript)

# Crate Links 

1. [Trait futures::future::Future](https://docs.rs/futures/0.2.0/futures/future/trait.Future.html)
2. [std::future::Future](https://doc.rust-lang.org/std/future/trait.Future.html)
3. [Futures - Crate](https://docs.rs/futures/0.3.5/futures/)

# Links 

1. See [example 1](tmp/example1.rs) for need of Futures. (Note it is pseudo-rustish code)


# Futures Concept

> Defintion: Expressing a value that is NOT ready/available.



## Java

## Javascript 

Concept of Event loops. Program is single-threaded.


# Futures in  Rust
- Rust has a crate for Futures. 
- Rust has Futures baked into the language from version **v1.36**


## Crate `futures`
> Abstractions  for async programming


### Trait
1. [Futures](https://docs.rs/futures/0.3.5/futures/prelude/trait.Future.html): Single values that are eventially produced when the value is computed. *(Similar to Promises in Javascript)*
2. [Streams](https://docs.rs/futures/0.3.5/futures/stream/trait.Stream.html): Series of values produced asynchronously 
3. [Sinks](https://docs.rs/futures/0.3.5/futures/sink/trait.Sink.html): Async. writing of data

### Modules
1. [Executors](https://docs.rs/futures/0.3.5/futures/executor/index.html): You define async. task as **futures**. Executors are responsible for running those async. tasks.


## Trait `futures::future::Future` 

```rust
pub trait Future {
    type Item;
    type Error;
    fn poll(
        &mut self, 
        cx: &mut Context
    ) -> Result<Async<Self::Item>, Self::Error>;
}
```

1. Item: Value the Future resolves to when it completes.
In `example1.rs`; it is analogous to `boolean` in `.and_then(|c, boolean| ...)`

2. Error: Errors is for each intermedite steps. 

3. Poll: Refer [`example2.rs`](/tmp/example2.rs) for code details. (Pseudo and not ACTUAL implementation)


## Trait `std::future::Future` 
