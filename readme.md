# A Rust wrapper for the termbox library.

[![Build Status](https://travis-ci.org/TyOverby/rust-termbox.svg)](https://travis-ci.org/TyOverby/rust-termbox)

[Crate documentation](http://apribadi.github.com/rust-termbox/).

The [original termbox library](https://github.com/nsf/termbox) was
created by nsf.

## Install

1. Install [original termbox library](https://github.com/nsf/termbox)
2. Add termbox as a cargo dependency

```
[dependencies.termbox]
git = "git@github.com:TyOverby/rust-termbox.git"
```


## Hello World example

```rust
extern crate std;
extern crate termbox;

use std::io::timer::sleep;
use std::time::duration::Duration;
use tb = termbox;

fn main() {
    tb::init();
    tb::print(1, 1, tb::Bold, tb::White, tb::Black, "Hello, world!");
    tb::present();
    sleep(Duration::milliseconds(1000));
    tb::shutdown();
}

```

## License
MIT
