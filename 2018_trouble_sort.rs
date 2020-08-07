use std::io;
#[allow(unused)]

fn main() {
	let cin = io::stdin();
 	let mut input = String::new();
 	cin.read_line(&mut input).unwrap();
 	let t: i32 = input.trim().parse().unwrap();

 	for i in 1..t+1 {
 		input = String::new();
 		cin.read_line(&mut input).unwrap();
 		let n: i32 = input.trim().parse().unwrap();

 		input = String::new();
 		cin.read_line(&mut input).unwrap();
		let mut values = input
			.split_whitespace()
			.map(|x| x.parse::<i32>())
			.collect::<Result<Vec<i32>, _>>()
			.unwrap() ;

		let mut done: bool = false;
		values = loop {
			done = true;
			for j in 0..n-2 {
				if values[j as usize] > values[j as usize +2] {
					done = false;
					let temp = values[j as usize];
					values[j as usize] = values[j as usize + 2];
					values[j as usize + 2] = temp;
				}
			}
			if done == true {
				break values;
			}
		};

		let mut pass: bool = true;
		let mut j: usize = 0;
		pass = loop {
			if values[j] > values[j + 1] {
				println!("Case #{}: {}", i, j);
				break false;
			}
			j += 1;
			if j as i32 == n-1 {
				break true;
			}
		};
		if pass {
			println!("Case #{}: OK", i);
		}
 	}
}