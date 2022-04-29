

pub fn bfunc() {
	println!("bfunc");
}


#[cfg(test)]
mod tests {
    use crate::mod0::*;

    #[test]
	fn test_bfunc() {
    	bfunc();
    	assert!(true);
    }
}
