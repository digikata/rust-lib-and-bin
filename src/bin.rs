extern crate mylib;

use mylib::test;
use mylib::mod0;
use mylib::mod1;
use mylib::mod2;

// module referenced from the binary package
mod bmod;

pub fn main() {
  let verstr =env!("CARGO_PKG_VERSION");
  println!("Version {}", verstr);
  
  test();

  mod0::bfunc();
  mod1::afunc();
  mod2::foofoo();
  // mod2::bar::bar(); // error: module bar is private
  mod2::barr();

  bmod::bmod_func();
}
