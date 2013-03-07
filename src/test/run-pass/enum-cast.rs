// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum A { A1, A2 }
enum B { B1=0, B2=2 }

fn main () {
    const c1: int = A2 as int;
    const c2: int = B2 as int;
    const c3: float = A2 as float;
    const c4: float = B2 as float;
    let a1 = A2 as int;
    let a2 = B2 as int;
    let a3 = A2 as float;
    let a4 = B2 as float;
    assert(c1 == 1);
    assert(c2 == 2);
    assert(c3 == 1.0);
    assert(c4 == 2.0);
    assert(a1 == 1);
    assert(a2 == 2);
    assert(a3 == 1.0);
    assert(a4 == 2.0);
}
