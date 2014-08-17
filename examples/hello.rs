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
