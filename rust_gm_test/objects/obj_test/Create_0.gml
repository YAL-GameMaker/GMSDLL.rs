rust_hello();
show_debug_message("1+2: " + string(rust_add_numbers(1, 2)));
show_debug_message("String: " + rust_get_string());
show_debug_message("Excite: " + rust_excite("Hello"));
show_debug_message("Dup: " + rust_dup_string("yo"));

var bufSize = 4;
var buf = buffer_create(bufSize, buffer_fixed, 1);
for (var i = 0; i < bufSize; i++) {
	buffer_poke(buf, i, buffer_u8, i);
}
var sum = rust_buffer_sum(buffer_get_address(buf), bufSize);
show_debug_message("Buffer sum: " + string(sum));

rust_buffer_inc(buffer_get_address(buf), bufSize, 3);
show_debug_message("After inc: " + string(buffer_peek(buf, 1, buffer_u8)));