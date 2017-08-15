
// reference privately
mod bar;

mod foo;

// 
pub use mod2::bar::bar as barr;


pub fn foofoo() {
	foo::afoo();
}
