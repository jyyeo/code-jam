#![allow(unused)]
// #[warn(non_snake_case)]

use std::io;

fn string_to_char(input: &str) -> Vec<char> {
	let chars: Vec<_> = input.chars().collect();
	let mut char_arr: Vec<char> = Vec::new();
	for ch in input.chars().collect::<Vec<char>>() {
		char_arr.push(ch);
	}
	return char_arr;
}

fn main() {
	let cin = io::stdin();
	let mut input = String::new();
	cin.read_line(&mut input).unwrap();
	let t: i32 = input.trim().parse().unwrap();

	for i in 1..t+1 {
		
		let mut input = String::new();
		cin.read_line(&mut input).unwrap();
		let values = input
			.split_whitespace()
			.map(|x| x.parse::<usize>())
			.collect::<Result<Vec<usize>, _>>()
			.unwrap();
		let r = values[0];
		let c = values[1];

		let mut grid = vec![vec!['.'; c]; r];
		for j in 0..r {
			let mut input = String::new();
			cin.read_line(&mut input);
			let char_vec: Vec<char> = input.chars().collect();
			let mut k = 0;
			for ch in char_vec {
				if (k == c) {
					break;
				}
				if (ch != '.') {
					grid[j][k] = ch;
				}
				k = k + 1;
			}
		}
		
		for j in 0..r {
			for k in 0..c {
				if (grid[j][k] != '.') {
					
				}
			}
		}
	}
}