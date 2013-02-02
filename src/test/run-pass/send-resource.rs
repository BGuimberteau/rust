// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::pipes::*;

struct test {
  f: int,
}

impl test : Drop {
    fn finalize(&self) {}
}

fn test(f: int) -> test {
    test {
        f: f
    }
}

pub fn main() {
    let (p, c) = stream();

    do task::spawn() {
        let (pp, cc) = stream();
        c.send(cc);

        let _r = pp.recv();
    }

    p.recv().send(test(42));
}
