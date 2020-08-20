use std::io;

fn main() {
	let cin = io::stdin();
	let mut input = String::new();
	let mut values: Vec<i32> = Vec::new();
	cin.read_line(&mut input).unwrap();
	let t: i32 = input.trim().parse().unwrap();
	let mut n: i32;
	let mut p: f32;
	for i in 1..t+1 {
		input.clear();
		values.clear();
		cin.read_line(&mut input).unwrap();
		values = input
			.split_whitespace()
			.map(|x| x.parse::<i32>())
			.collect::<Result<Vec<i32>, _>>()
			.unwrap() ;
		n = values[0];
		p = values[1] as f32;
		
		let mut curr_p: f32 = 0.0;

		let mut w = vec![0.0; n as usize];
		let mut h = vec![0.0; n as usize];
		let mut max_add = vec![0.0; n as usize];
		let mut min_add: f32 = 0.0;
		let mut idx = 0 as usize;

		for _ in 0..n {
			input.clear();
			values.clear();
			cin.read_line(&mut input).unwrap();
			values = input
				.split_whitespace()
				.map(|x| x.parse::<i32>())
				.collect::<Result<Vec<i32>, _>>()
				.unwrap() ;

			w[idx] = values[0] as f32;
			h[idx] = values[1] as f32;
			curr_p += w[idx] * 2.0;
			curr_p += h[idx] * 2.0;
			let smallest_cut = if w[idx] > h[idx] {h[idx]*2.0} else {w[idx]*2.0};
			min_add = if idx == 0 {smallest_cut} 
					  else {
					  	if min_add > smallest_cut {smallest_cut} 
					  	else {min_add}};
			let addition = (w[idx]*w[idx] + h[idx]*h[idx]).sqrt() * 2.0;
			max_add[idx] = if idx == 0 {addition} else {max_add[idx-1] + addition};

			idx += 1;
		}
		let diff = p - curr_p;
		if min_add < diff && diff < max_add[n as usize - 1] {
			println!("Case #{}: {:.6}", i, p);
			continue;
		}
		else if diff < min_add {
			println!("Case #{}: {:.6}", i, curr_p);
		}
		else {
			println!("Case #{}: {:.6}", i, curr_p + max_add[n as usize -1]);
		}
	}
}