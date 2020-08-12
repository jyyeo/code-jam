#![allow(unused)]

use std::io;

fn gcd(a: i32, b: i32) -> i32 {
	if (a == 0) {
		return b;
	}
	gcd(b%a, a)
}

fn simplify_frac(a: i32, b: i32) -> (i32, i32) {
	let common_denom = gcd(a, b);
	(a/common_denom, b/common_denom)
}

fn smaller_frac(temp_numer: i32, temp_denom: i32, 
	old_numer: i32, old_denom: i32) -> (i32, i32) {
	if (temp_numer/temp_denom < old_numer/old_denom) {
		return (temp_numer, temp_denom);
	}
	else {
		return (old_numer, old_denom);
	}
}

fn larger_frac(temp_numer: i32, temp_denom: i32, 
	old_numer: i32, old_denom: i32) -> (i32, i32) {
	if (temp_numer/temp_denom > old_numer/old_denom) {
		return (temp_numer, temp_denom);
	}
	else {
		return (old_numer, old_denom);
	}
}

fn use_upperbound(lower_numer: i32, lower_denom: i32, 
	upper_numer: i32, upper_denom: i32) -> (i32, i32) {
	let upper_denom = if upper_numer == 1 {upper_denom+1} else {upper_denom};
	let upper_numer = if upper_numer == 1 {upper_numer} else {upper_numer-1};
	let (upper_numer, upper_denom) = if lower_numer/lower_denom > upper_numer/upper_denom
		{(0,0)} else {(upper_numer, upper_denom)};
	return (upper_numer, upper_denom);
}

fn update_bounds(grid: &Vec<Vec<i32>>, n: usize) -> (i32, i32, i32, i32) {
	let mut upper_numer = 100;
	let mut upper_denom = 1;
	let mut lower_numer = 1;
	let mut lower_denom = 100;
	let mut temp_numer: i32;
	let mut temp_denom: i32;

	for j in 0..n-1 {
		let a = grid[j][0];
		let b = grid[j][1];
		let c = grid[j+1][0];
		let d = grid[j+1][1];


		if (a == c) { //d must be > b because strictly larger
			continue;
		}
		if (b == d) { //c must be > a because strictly larger
			continue;
		}
		if (c > a && d > b) {
			continue;
		}
		// c != a AND d != b
		let (mut temp_numer, mut temp_denom) = simplify_frac(c-a, b-d);

		let x: (i32,i32) = if b - d < 0 {
			larger_frac(temp_numer,temp_denom, lower_numer, lower_denom)
		} else {
			(lower_numer, lower_denom)
		};
		lower_numer = if b-d < 0 {x.0} else {lower_numer};
		lower_denom = if b-d < 0 {x.1} else {lower_denom};

		let x: (i32,i32) = if b - d > 0 {
			smaller_frac(temp_numer,temp_denom, upper_numer, upper_denom)
		} else {
			(upper_numer, upper_denom)
		};
		upper_numer = if b-d > 0 {x.0} else {upper_numer};
		upper_denom = if b-d > 0 {x.1} else {upper_denom};

		if j == n-2 {
			return (lower_numer, lower_denom, upper_numer, upper_denom);
		}
	}	
	return (lower_numer, lower_denom, upper_numer, upper_denom);
}

fn main() {
	let cin = io::stdin();
	let mut input = String::new();
	cin.read_line(&mut input).unwrap();
	let t: i32 = input.trim().parse().unwrap();

	for i in 1..t+1 {
		let mut input = String::new();
		cin.read_line(&mut input).unwrap();
		let n: usize = input.trim().parse().unwrap();
		
		let mut grid = vec![vec![0; 3]; n];
		for j in 0..n {
			let mut input = String::new();
			cin.read_line(&mut input).unwrap();
			let values = input
				.split_whitespace()
				.map(|x| x.parse::<i32>())
				.collect::<Result<Vec<i32>, _>>()
				.unwrap() ;
			grid[j][0] = values[0];
			grid[j][1] = values[1];

		}

		let mut upper_numer = 100i32;
		let mut upper_denom = 1i32;
		let mut lower_numer = 1i32;
		let mut lower_denom = 100i32;
		let temp_numer: i32;
		let temp_denom: i32;

		let (lower_numer, lower_denom, upper_numer, upper_denom) = update_bounds(&grid, n);

		let lowerbound = lower_numer as f32/lower_denom as f32;
		let upperbound = upper_numer as f32/upper_denom as f32;
		let wc = 0i32;
		let wj = 0i32;
		if (lowerbound >= upperbound) {
			println!("Case #{}: IMPOSSIBLE", i);
		}
		else {
			if (lowerbound < 1.0 && upperbound > 1.0) {
				let wc = 1i32;
				let wj = 1i32;
				println!("Case #{}: {} {}", i, wc, wj);
			}
			else if (lowerbound >= 1.0) {
				let wc = lower_denom;
				let wj = lower_numer + 1;
				println!("Case #{}: {} {}", i, wc, wj);
			}
			else { // LB and UB < 1.0
				let (wj, wc) = use_upperbound(lower_numer, lower_denom, upper_numer, upper_denom);
				if (wj, wc) == (0,0) {
					println!("Case #{}: IMPOSSIBLE", i);
				}
				else {
					println!("Case #{}: {} {}", i, wc, wj);
				}
			}
		}
	}
}