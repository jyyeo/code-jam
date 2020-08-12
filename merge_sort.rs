use std::io;

fn merge(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
	let n: usize = a.len() + b.len();
	a.reverse();
	b.reverse();
	let mut temp = vec![0; n];
	for i in 0..n {
		if a.is_empty() {
			temp[i] = b.pop().unwrap();
			continue;
		}
		if b.is_empty() {
			temp[i] = a.pop().unwrap();
			continue;
		}
		if a.last() < b.last() {
			temp[i] = a.pop().unwrap();
		}
		else {
			temp[i] = b.pop().unwrap();
		}
	}
	return temp;
}

fn merge_sort(values: &mut Vec<i32>) -> Vec<i32> {
	let n: usize = values.len();
	if n == 1 {
		return values.to_vec();
	}
	if n == 2 {
		if values[0] > values[1] {
			values.swap(0,1);
		}
		return values.to_vec();
	}

	let mut first_half = values[0..=n/2].to_vec();
	let mut second_half = values[n/2+1..].to_vec();
	let mut sorted_first_half = merge_sort(&mut first_half);
	let mut sorted_second_half = merge_sort(&mut second_half);

	let merged_list = merge(&mut sorted_first_half, &mut sorted_second_half);
	
	return merged_list;
}

fn main() {
	let cin = io::stdin();
	let mut input = String::new();
	cin.read_line(&mut input).unwrap();
	let mut values = input
		.split_whitespace()
		.map(|x| x.parse::<i32>())
		.collect::<Result<Vec<i32>, _>>()
		.unwrap() ;

	let sorted_values = merge_sort(&mut values);

	println!("{:?}", sorted_values);
}