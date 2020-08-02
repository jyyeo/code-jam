use std::io;
// use std::cmp;
use std::iter;

// fn print_line(line: &Vec<Vec<i32>>, n:usize) {
// 	for j in 0..n {
// 		println!("{} {} {}", line[0][j], line[1][j], line[2][j], line[3][j]);
// 	}
// }

fn print_line(line: &Vec<u16>, n: usize) {
	for i in 0..n {
		print!("{}", line[i]);
	}
	print!("\n");
}

// fn index_of_max(values: &[i32]) -> Option<usize> {
// 	values
// 		.iter()
// 		.enumerate()
// 		.max_by_key(|&(_idx, &val)| val)
// 		.map(|(idx, _val)| idx)
// }

fn eval_line () {
	let min
}

fn insert_hoomans(line: &Vec<u16>, count: usize, n: usize) -> (usize, usize) {
	if count == 0 {
		let index = line.iter().position(|&x| x == n);
		let mut iter = 1;
		let Ls = loop {
			if line[index-iter] == 0 {
				break iter;
			}
			else {
				iter += 1;
			}
		};
		iter = 1;
		let Rs = loop {
			if line[index+iter] == 0 {
				break iter;
			}
			else {
				iter += 1;
			}
		};
		if Ls >= Rs {
			return (Ls, Rs);
		}
		else {
			return (Rs, Ls);
		}
	}

	else {

		let (Ls, Rs) = insert_hoomans(&line, count-1, n);
	}	
}

fn main() {
	let cin = io::stdin();
	let mut input = String::new();
	cin.read_line(&mut input).unwrap();
	let t: i32 = input.trim().parse().unwrap();

	for i in 1..t+1 {
		input = String::new();
		cin.read_line(&mut input).unwrap();
		let values = input
			.split_whitespace()
			.map(|x| x.parse::<usize>())
			.collect::<Result<Vec<usize>, _>>()
			.unwrap() ;
		let n = values[0];
		let k = values[1];

		let mut line = vec![0; n];
		print_line(&line, n);

		

		// let mut line = vec![vec![0; n]; 4];

		// let mut row = 0;
		// let mut col = 0;
		// line = loop {
		// 	line[0][col] = 0;
		// 	line[1][col] = row; //Ls
		// 	line[2][col] = n - row; // Rs
		// 	line[3][col] = cmp::min(line[0][col], line[1][col]);
		// 	col += 1;
		// 	if col == n {
		// 		break line;
		// 	}
		// };

		// println!("{:?}", line[3]);
		// let count: usize = 0;
		// loop {
		// 	let max_index = index_of_max(&line[3]).unwrap_or(0);

		// 	count += 1;
		// 	if count == n {break; }
		// }
	}
}

