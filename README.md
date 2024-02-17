# Radar

📡 a ligthweight alternative to `WatchTower`

![alt Experimental](https://img.shields.io/badge/Type-Experimental-red.svg)
![alt Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![alt Binary](https://img.shields.io/badge/Architecture-binary-green.svg)
![alt Failed](https://img.shields.io/badge/Failed-👎_0-red.svg)
![alt Passed](https://img.shields.io/badge/Passed-👍_0-green.svg)
![alt Version](https://img.shields.io/badge/version-0.0.1_BETA-blue.svg)

```console

    ░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░
    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
    ░▒▓███████▓▒░░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓███████▓▒░
    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░

```

# Introduction

The [Watchtower](https://github.com/containrrr/watchtower/) is a very popular project that is used to keep an
eye on running containers on a host. It automatically redeploys containers if a newer image version is found.

I wanted to create a much more simplified and lightweight alternative to `watchtower` and hence `radar` was born!

## How to build

Just use the standard Rust `cargo` toolchain to build from source. Alternatively to build a statically
linked binary, just run the handy build script `./build.sh`.

```console
$ cargo build
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    Running `target/debug/ron`
....
```

## How to use

TODO

# Version

0.0.1-ALPHA

## Contributors

- [anharmiah](https://github.com/anharhussainmiah) Anhar Miah - creator, maintainer
