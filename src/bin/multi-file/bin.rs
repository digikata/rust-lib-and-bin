
use thelib::test;
use thelib::mod0;
use thelib::mod1;
use thelib::mod2;

// module referenced from the binary package
mod bmod;

pub fn main() {
  let verstr =env!("CARGO_PKG_VERSION");
  println!("Version {}", verstr);
  println!("foo: {}", foo());
  test();

  mod0::bfunc();
  mod1::afunc();
  mod2::foofoo();
  // mod2::bar::bar(); // error: module bar is private
  mod2::barr();

  bmod::bmod_func();
}

fn foo() -> usize {
    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() -> Result<(), String> {
        let x = 1usize;
        assert_eq!(x, foo());
        Ok(())
    }
}
