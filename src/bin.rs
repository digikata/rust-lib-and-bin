extern crate mylib;

use mylib::test;
use mylib::mod0;
use mylib::mod1;
use mylib::mod2;

pub fn main() {
  test();

  mod0::bfunc();
  mod1::afunc();
  mod2::foofoo();
  // mod2::bar::bar(); // error: module bar is private
  mod2::barr();
}
