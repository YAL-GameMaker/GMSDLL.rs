# Rust x GameMaker example
This is an example of how to make native extensions for GameMaker in Rust!

## Points of interest

### Export macro
This lets you do
```rust
export! {
	fn rust_get_string() -> GMStr {
		rets("This string comes from Rust!")
	}
	fn rust_add_numbers(left: f64, right: f64) -> f64 {
		left + right
	}
}
```
instead of 
```rust
#[unsafe(no_mangle)]
extern "C" fn rust_get_string() -> GMStr {
	rets("This string comes from Rust!")
}
#[unsafe(no_mangle)]
extern "C" fn rust_add_numbers(left: f64, right: f64) -> f64 {
	left + right
}
```
similar to how I always have a `dllx` "decorator" in my C++ extensions.

### Receiving and returning numbers
This one's easy, you specify `f64` for argument types and/or return type and that's it.

### Receiving and returning strings
This one's quirkier due to FFI philosophy differences between languages -
Rust generally expects (`CString.into_raw`) the returned C strings to be `free`d
by the caller program while GM makes a copy of the returned string without `free`ing it.

This means that you must store the string you're about to return somewhere
so that it does not get `free`d on function exit.

For this reason I'm including `gm::gets` (create a Rust `String` from a GM string)
and `gm::rets` (convert a Rust `&str` to a GM string for immediate return) helper functions.

### Working with buffers
On GM side you mark the argument as a "string" (pointer, really),
pass a `buffer_get_address` to the native extension,
and that's it! This is an actual pointer to buffer data so you can also write to the buffer,
but note that you'll need to call `buffer_set_used_size` afterwards if you're populating
a freshly made buffer with data.

## Special thanks
- [Jacob](https://jacobsdot.tumblr.com) for answering my general "and how do you do X here" Rust questions
- [Dashiell](https://dashiellwas.cool) for explaining the particularities of Rust's `static` and a few other bits