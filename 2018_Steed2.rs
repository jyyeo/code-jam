use std::io;

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
			.map(|x| x.parse::<f64>())
			.collect::<Result<Vec<f64>, _>>()
			.unwrap() ;

		let mut temp_max = 0f64;
		let mut count = 0f64;
		let max_time = loop {
			input = String::new();
			cin.read_line(&mut input).unwrap();
			let data = input
				.split_whitespace()
			.map(|x| x.parse::<f64>())
			.collect::<Result<Vec<f64>, _>>()
			.unwrap() ;
			let time: f64 = (values[0] - data[0]) / data[1];
			temp_max = if time > temp_max {time} else {temp_max};

			count = count + 1.0;
			if count == values[1] {
				break temp_max;
			}
		};

		println!("Case #{}: {:.6}", i, values[0]/max_time);
	}
}