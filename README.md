# SIMUWAERM

Simuwaerm is a simple heat simulation written in pure Rust. 

![screenshot](https://fi-le.net/images/simuwaerm.png?raw=true)

## How it works
The border is fixed, the middle tiles simulate a conductive plate. If you leave it be, it numerically solves the border condition, so that the middle has Lacplacian 0 everywhere. You can manually add heat or cold with keys `h` and `k`.  λ is the uniform conductivity and, in reality, satisfies 0 < λ < 1. (Feel free to break this boring limiting condition however.)

The language can be switched to German with 'l'.

## Installation
You can download a binary ![here](https://fi-le.net/warehouse/simuwaerm.zip).

If you want to build it yourself, you can run:
```bash
git clone https://github.com/file-acomplaint/simuwaerm.git
cd simuwaerm
cargo run --release
```
