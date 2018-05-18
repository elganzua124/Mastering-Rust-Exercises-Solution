// byte strings (streams of bytes) vectores Vec<u8>  arays &[u8] (a byte slice)
// http://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt


#[cfg(test)] #[allow(unused_variables)]
mod tests {
	#[test]
	#[should_panic(expected="Utf8Error")]
	fn test_non_utf8_1() {
		let non_utf8_1 = vec![104, 101, 0xFF, 108, 111];

		String::from_utf8(non_utf8_1).unwrap();
	}
	#[test]
	fn test_non_utf8_2() {
		let non_utf8_2 = vec![0xFC, 80, 80, 80, 80, 0xAF];

		assert_eq!(String::from_utf8(non_utf8_2).is_err(), true);
	}
}

fn print_slice_string(string: &str) {
	println!("{}", string);
}

fn sum_slice(slice: &[usize]) -> usize{
	let mut sum = 0;
	for i in slice.iter() {
		sum+=i;
	}
	sum
}

fn main() {
	print_slice_string("hholiuboluhbol");
	print_slice_string(&String::from("hholiuboluhbol")); // dereferencing

	let phrase = "You are a Rust coder, Harry.";

	// silly way
	for (i, word) in phrase.split(" ").enumerate() {
		if i == 1 { // second word
			println!("{}", word);
			break;
		}
	}
	// one-liner solution
	println!("{}", phrase.split(" ").collect::<Vec<&str>>()[1] );

	let phrase_to_string = phrase.to_string();

	// also an one-liner solution but with string instead (as asked)
	println!("{}", phrase_to_string.split(" ").collect::<Vec<&str>>()[1] );

	let array : [usize;10] = [1,34,665,2,0,1,55,76,54,6];
	println!("{}",sum_slice(&array[1..9]));
	println!("{}",sum_slice(&array));

}