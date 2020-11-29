use std::io;
#[allow(unused)]

fn index_of_min(values: &[i32]) -> Option<usize> {
	values
		.iter()
		.enumerate()
		.min_by_key(|&(_idx, &val)| val)
		.map(|(idx, _val)| idx)
}

fn first_b_smallest (vec: &mut Vec<Vec<i32>>, r: i32, b: i32, c: i32) -> Vec<i32> {
	let mut output = Vec::new();
	let mut pos = Vec::new();
	let mut curr_col = Vec::new();
	let mut unique = Vec::new();
	for _ in 0..b {
		curr_col.clear();
		for j in 0..c {
			if vec[j as usize].len() != 0 {
				curr_col.push(vec[j as usize][0]);
			}
		}
		print!("curr_col: {:?} ", curr_col);
		let row = index_of_min(&mut curr_col).unwrap();

		pos.push(row);
		if unique.iter().any(|&n| n!=row) {
			unique.push(row);
		}

		output.push(vec[row].remove(0));
	} 
	print!("pos: {:?} ", pos);
	print!("output: {:?} ", output);
	return output;
}

fn main() {
	let cin = io::stdin();
 	let mut input = String::new();
 	cin.read_line(&mut input).unwrap();
 	let t: i32 = input.trim().parse().unwrap();

 	for i in 1..t+1 {
 		input.clear();
 		cin.read_line(&mut input).unwrap();
 		let mut values = input
			.split_whitespace()
			.map(|x| x.parse::<i32>())
			.collect::<Result<Vec<i32>, _>>()
			.unwrap() ;
		let r: i32 = values[0];
		let b: i32 = values[1];
		let c: i32 = values[2];

		let mut vec: Vec<Vec<i32>> = Vec::with_capacity(c as usize * b as usize);
		for _ in 0..c {
			values.clear();
			input.clear();
			cin.read_line(&mut input).unwrap();
			values = input
				.split_whitespace()
				.map(|x| x.parse::<i32>())
				.collect::<Result<Vec<i32>, _>>()
				.unwrap() ;

			let mut row = Vec::new();
			for k in 0..values[0] {
				row.push((k+1) * values[1] + values[2]);
			}
			vec.push(row);
			print!("vec: {:?} ", vec);
		}

		let min_time = first_b_smallest(&mut vec, r, b, c);

		println!("Case #{}: {}", i, min_time.iter().max().unwrap());
 	}
}