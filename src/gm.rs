use core::ffi::c_char;
use std::{ffi::{CStr, CString}, ptr};

pub type GMStr = *const c_char;
pub fn gets(s: GMStr) -> String {
	unsafe {
		let cs = CStr::from_ptr(s);
		cs.to_str().unwrap().to_owned()
	}
}

pub fn rets(s: &str) -> GMStr {
	static mut STORED: *mut c_char = ptr::null_mut();
	unsafe {
		// GM copies returned strings without free()-ing,
		// so store the converted string until this gets called again
		if !STORED.is_null() {
			STORED.drop_in_place();
			STORED = ptr::null_mut();
		}
		STORED = CString::new(s).unwrap().into_raw();
		STORED
	}
}

/*
macro_rules! export_one {
	(
		$vis:vis fn $name:ident(
			$($aname:ident : $atype:ty),*
		) -> $ret:ty $block:block
	) => {
		#[unsafe(no_mangle)]
		$vis extern "C" fn $name($($aname: $atype),*) -> $ret $block
	};
}
*/
macro_rules! export {
	(
		$(
			$vis:vis fn $name:ident(
				$($aname:ident : $atype:ty),*
			) -> $ret:ty $block:block
		)*
	) => {
		$(
			#[unsafe(no_mangle)]
			$vis extern "C" fn $name($($aname: $atype),*) -> $ret $block
		)*
	};
}