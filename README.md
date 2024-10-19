<div align="center">

# simple-semaphore

[![Docs.rs Badge](https://img.shields.io/badge/docs.rs-simple-semaphore-61c192.svg)](https://docs.rs/simple-semaphore)
[![Crates.io Badge](https://img.shields.io/crates/v/simple-semaphore.svg?color=fe7d37)](https://crates.io/crates/simple-semaphore)
[![CI Badge](https://github.com/kkrypt0nn/simple-semaphore/actions/workflows/ci.yml/badge.svg)](https://github.com/kkrypt0nn/simple-semaphore/actions)
[![Dependency Status Badge](https://deps.rs/repo/github/kkrypt0nn/simple-semaphore/status.svg)](https://deps.rs/repo/github/kkrypt0nn/simple-semaphore)

[![Discord Server Badge](https://img.shields.io/discord/739934735387721768?logo=discord)](https://discord.gg/mTBrXyWxAF)
[![Last Commit Badge](https://img.shields.io/github/last-commit/kkrypt0nn/simple-semaphore)](https://github.com/kkrypt0nn/simple-semaphore/commits/main)
[![Conventional Commits Badge](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org/en/v1.0.0/)

</div>

---

A lightweight implementation of a Semaphore in Rust.

## Getting Started

### Installation

If you want to use this library for one of your projects, you can install it like any other Rust library

```shell
cargo add simple-semaphore
```

### Example Usage

Here a basic example on how to use the crate:
```rs
use simple_semaphore;
use std::thread;
use std::{sync::Arc, time::Duration};

fn main() {
    let semaphore = simple_semaphore::Semaphore::new(2);
    for _ in 0..5 {
        let semaphore = Arc::clone(&semaphore);
        thread::spawn(move || {
            let permit = semaphore.acquire();
            thread::sleep(Duration::from_millis(500));
            drop(permit);
        });
    }
    thread::sleep(Duration::from_millis(3000));
}
```

## License

This library was made with 💜 by Krypton and is under the [MIT License](./LICENSE.md).
