use std::io;

fn index_of_max(values: &[i32]) -> Option<usize> {
	values
		.iter()
		.enumerate()
		.max_by_key(|&(_idx, &val)| val)
		.map(|(idx, _val)| idx)
}

fn add_char(mut queue: String, ch: char) -> String {
	queue.push(ch);
	return queue.to_string();
}

fn main() {
	let cin = io::stdin();
	let mut input = String::new();
	cin.read_line(&mut input).unwrap();
	let t: i32 = input.trim().parse().unwrap();

	for i in 1..t+1 {
		input = String::new();
		cin.read_line(&mut input).unwrap();
		// let n: usize = input.trim().parse().unwrap();
		let mut input = String::new();
		cin.read_line(&mut input).unwrap();
		let mut values = input
			.split_whitespace()
			.map(|x| x.parse::<i32>())
			.collect::<Result<Vec<i32>, _>>()
			.unwrap() ;
		// for j in 0..n {
		// 	print!("{} ", values[j]);
		// }
		let mut queue = String::new();
		let output = loop {
			let max_index = index_of_max(&values).unwrap_or(0);
			queue.push((max_index as u8 + 65u8) as char);
			values[max_index] = values[max_index] - 1;
			let sum: i32 = values.iter().sum();

			if sum == 0 {
				// println!("{}", queue);
				break queue;
			}

			queue = if sum%2 == 0 {add_char(queue, ' ')} else {queue};
			// println!("{}", queue);

		};
		println!("Case #{}: {}", i, output);
	}	
}