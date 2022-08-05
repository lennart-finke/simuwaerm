# SIMUWAERM
![Crates.io](https://img.shields.io/crates/v/simuwaerm?logo=rust&style=flat-square)![GitHub](https://img.shields.io/github/license/file-acomplaint/simuwaerm?color=pink&style=flat-square)

Simuwaerm is a simple heat simulation written in pure Rust. 

![screenshot](https://fi-le.net/images/simuwaerm2.png?raw=true)


## How it works
The border is fixed, the middle tiles simulate a conductive plate. If you leave it be, it numerically solves the border condition, so that the middle has Laplacian 0 everywhere. You can manually add heat or cold with keys `h` and `k`.  

`λ` is the uniform conductivity and, in reality, satisfies `0 < λ < 1`. (Feel free to break this boring limiting condition however.)

The language can be switched to German with `l`.

## Installation
You can download a binary ![here](https://github.com/file-acomplaint/simuwaerm/releases).

If you have `cargo`, simply execute:
```bash
cargo install simuwaerm
```

## Compatibility
Simuwaerm runs on UNIX and Windows, about equally well. Above is a screenshot on KDE Konsole, below on the Command Prompt:

![screenshot](https://fi-le.net/images/simuwaerm3.png?raw=true)

If you happen to use a niche operating system, say macOS, please try compiling and send me a funny caption of what happens. (To clarify, I love you mac people just as much, I just don't have the hardware to try it out. Any help with testing is appreciated!)
