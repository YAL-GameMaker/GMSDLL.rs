use crate::gm::*;

#[macro_use]
mod gm;

export! {
	fn rust_hello() -> () {
		println!("Hello from Rust!");
	}
	fn rust_get_string() -> GMStr {
		rets("This string comes from Rust!")
	}
	fn rust_add_numbers(left: f64, right: f64) -> f64 {
		left + right
	}
	fn rust_add_strings(left: GMStr, right: GMStr) -> GMStr {
		rets(&(gets(left) + &gets(right)))
	}
	fn rust_dup_string(s: GMStr) -> GMStr {
		rets(&(gets(s) + &gets(s)))
	}
	fn rust_excite(s: GMStr) -> GMStr {
		rets(&(gets(s) + "!"))
	}
	fn rust_buffer_sum(buf: *const u8, size: f64) -> f64 {
		let mut result:f64 = 0.0;
		for i in 0 .. (size as i32) {
			unsafe {
				let ofs = i.try_into().unwrap();
				result += *buf.add(ofs) as f64;
			}
		}
		result
	}
	fn rust_buffer_inc(buf: *mut i8, size: f64, delta: f64) -> f64 {
		for i in 0 .. (size as i32) {
			unsafe {
				let ptr = buf.add(i.try_into().unwrap());
				*ptr = *ptr + (delta as i8);
			}
		}
		0.0
	}
}
