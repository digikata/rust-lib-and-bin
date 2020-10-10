
// reference privately
mod bar;

mod foo;

// 
pub use bar::bar as barr;


pub fn foofoo() {
	foo::afoo();
}
