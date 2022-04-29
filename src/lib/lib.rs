
pub fn test() {
  println!("Test");
}

pub mod mod0;
pub mod mod1;
pub mod mod2;
//pub use mod2::bar;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
