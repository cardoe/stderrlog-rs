// Copyright 2018 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate log;

pub mod bar;
mod foo;

pub fn libfn() {
    println!("Entered {}", module_path!());
    trace!("{} - trace message", module_path!());
    debug!("{} - debug message", module_path!());
    info!("{} - info message", module_path!());
    warn!("{} - warn message", module_path!());
    error!("{} - warn message", module_path!());

    foo::foofn();
}
