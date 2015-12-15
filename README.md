Async-Await
==========

[![Crates.io](http://img.shields.io/crates/v/async-await.svg)](https://crates.io/crates/async-await)

Just two macros to emulate a simple Async and Await using Futures (provided by eventual).

## Usage

This is available in `crates.io`. Add this to your `Cargo.toml`:

```
[dependencies]
async-await = "0.1.1"
```

## Example

Here is a simple example, you need to do the `#[macro_use]` and `use async_await::*;` because of the expansion of the macros :)

```rust
#[macro_use]
extern crate async_await;

use async_await::*;

fn main() {
    let computation = async!{"Hello world!"};
    println!("{}", await!(computation));
}
```

Another example using hyper, a shared client and a block in async :)

```rust
#[macro_use]
extern crate async_await;
extern crate hyper;

use std::io::Read;
use std::sync::Arc;

use async_await::*;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    let client = Arc::new(Client::new());

    let client_comp = client.clone();
    let computation = async!{{
        let mut res = client_comp.get("http://rust-lang.org/")
            .header(Connection::close())
            .send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }};

    println!("Before await!");
    println!("{}", await!{computation});
    println!("After await!");
}
```

You can also provide a default value in case that the computation fails:

```rust
#[macro_use]
extern crate async_await;

use async_await::*;

fn main() {
    let computation = async!{panic!(":()")};
    assert_eq!("Recovered!", await!{computation, "Recovered!"});
}
```
