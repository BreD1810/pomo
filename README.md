Pomo
===
[![pipeline status](https://git.bredley.co.uk/BreD/pomo/badges/master/pipeline.svg)](https://git.bredley.co.uk/BreD/pomo/-/commits/master)[![crates.io](https://img.shields.io/crates/v/pomo)](https://crates.io/crates/pomo)

A [Pomodoro](https://francescocirillo.com/pages/pomodoro-technique) CLI tool written in Rust.

![](pomo.gif)

Installation
---
To install, run:
```
cargo install pomo
```


Usage
---
```
pomo

USAGE:
    pomo [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -l, --long <long_break>      Number of minutes for a long break. [default: 20]
    -p, --pomdoro <pomodoro>     Number of minutes for each Pomodoro. [default: 25]
    -s, --short <short_break>    Number of minutes for a short break. [default: 5]
```
