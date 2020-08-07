use std::io;
#[allow(unused)]

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

	let mut first_half = values[0..n/2+1].to_vec();
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
 	let t: i32 = input.trim().parse().unwrap();

 	for i in 1..t+1 {
 		input = String::new();
 		cin.read_line(&mut input).unwrap();
 		let n: i32 = input.trim().parse().unwrap();

 		input = String::new();
 		cin.read_line(&mut input).unwrap();
		let values = input
			.split_whitespace()
			.map(|x| x.parse::<i32>())
			.collect::<Result<Vec<i32>, _>>()
			.unwrap() ;

		let mut even = Vec::new();
		let mut odd = Vec::new();
		for j in 0..n {
			if j%2 == 0 {
				even.push(values[j as usize]);
			}
			else {
				odd.push(values[j as usize]);
			}
		}

		let mut sorted_even = merge_sort(&mut even.to_vec());
		let mut sorted_odd = merge_sort(&mut odd.to_vec());

		sorted_even.reverse();
		sorted_odd.reverse();
		let mut sorted_correctly: bool = true;
		let mut j: usize = 0;
		let mut sorted = vec![0i32; n as usize];
		for _ in 0..n {
			if sorted_even.is_empty() {
				sorted[j] = sorted_odd.pop().unwrap();
			    if sorted[j] < sorted[j-1] {
					println!("Case #{}: {}", i, j-1);
					sorted_correctly = false;
					break;
				}
				continue;
			}
			if sorted_odd.is_empty() {
				sorted[j] = sorted_even.pop().unwrap();
				if sorted[j] < sorted[j-1] {
					println!("Case #{}: {}", i, j-1);
					sorted_correctly = false;
					break;
				}
				continue;
			}
			if j%2 == 0 {
				sorted[j] = sorted_even.pop().unwrap();
			}
			else {
				sorted[j] = sorted_odd.pop().unwrap();
			}
			if j != 0 {
				if sorted[j] < sorted[j-1] {
					println!("Case #{}: {}", i, j-1);
					sorted_correctly = false;
					break;
				}
			}
			j += 1;
		}
		if sorted_correctly {
			println!("Case #{}: OK", i);
		}
 	}
}