use std::io;
#[allow(unused)]

fn print_line(p: &Vec<char>) {
	let n = p.len();
	for i in 0..n {
		print!("{}", p[i]);
	}
	print!(" ");
}

fn last_s(p: &Vec<char>, n: usize) -> usize {
	let mut j = n-1;
	loop {
		if p[j] == 'S' && p[j-1] == 'C' {
			return j;
		}
		j -= 1;
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
			.map(|x| x.parse::<String>())
			.collect::<Result<Vec<String>, _>>()
			.unwrap() ;
		let d = values[0].parse::<i32>().unwrap();
		let mut p: Vec<char> = values[1].chars().collect();

		// determine if it is possible
		let min_possible_dmg = p.iter().filter(|&x| *x == 'S').count() as i32;
		if min_possible_dmg > d {
			println!("Case #{}: IMPOSSIBLE", i);
			continue;
		}

		// assuming it is possible
		let mut count = 0;
		let num_switches = loop {
			// print_line(&p);
			let mut temp = 0;
			let mut j = 0;
			let mut dmg = 1;
			let total_dmg = loop {
				if p[j] == 'C' {
					dmg *= 2;
				}
				if p[j] == 'S' {
					temp += dmg;
				}
				j += 1;
				if j == (p.len() as usize) {
					break temp;
				}
			};
			if total_dmg <= d {
				break count;
			}
			else {
				// switch places. 
				let last_s_idx = last_s(&p, p.len() as usize);
				p[last_s_idx] = 'C';
				p[last_s_idx-1] = 'S';
				count += 1;
			}
		};
		println!("Case #{}: {}", i, num_switches);
 	}
 }