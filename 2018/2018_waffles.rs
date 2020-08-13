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
		let values = input
			.split_whitespace()
			.map(|x| x.parse::<i32>())
			.collect::<Result<Vec<i32>, _>>()
			.unwrap() ;

		let mut grid = vec![vec!['.'; values[1] as usize]; values[0] as usize];
		let mut per_row = vec![0; values[0] as usize];
		let mut hori_arr = vec![0; values[0] as usize];
		let mut vert_arr = vec![0; values[1] as usize];
		let mut row_idx: usize = 0;
		let mut col_idx: usize = 0;
		for r in 0..values[0] {
			input = String::new();
			cin.read_line(&mut input).unwrap();
			let row: Vec<char> = input.chars().collect();
			
			grid[row_idx] = row;
			
			per_row[row_idx] = grid[row_idx].iter().filter(|&n| *n == '@').count() as i32;
			if r == 0 {
				hori_arr[row_idx] = per_row[row_idx];
			}
			else {
				hori_arr[row_idx] = hori_arr[row_idx-1] + per_row[row_idx]
			}

			col_idx = 0;
			for _ in 0..values[1] {
				if grid[row_idx][col_idx] == '@' {
					vert_arr[col_idx] += 1;
				}
				col_idx += 1;
			}

			if r == values[0]-1 {
				col_idx = 0;
				for c in 0..values[1] {
					if c != 0 {
						vert_arr[col_idx] += vert_arr[col_idx-1];
					}
					col_idx += 1;
				}
			}
			row_idx += 1;
		}
		// print!("hori_arr: {:?}", hori_arr);
		// print!("vert_arr: {:?}", vert_arr);

		let sum: i32 = grid.iter().flatten().filter(|&n| *n == '@').count() as i32;
		let num_pieces: i32 = (values[2]+1) * (values[3]+1);
		if sum % num_pieces != 0 {
			println!("Case #{}: IMPOSSIBLE", i);
			continue;
		}

		let choc_per_row: i32 = sum / (values[2]+1);
		let choc_per_col: i32 = sum / (values[3]+1);
		// let choc_per_pc: i32 = sum / num_pieces;
		let mut row_interval = vec![vec![0, 2usize]; values[2] as usize +1];
		let mut col_interval = vec![vec![0, 2usize]; values[3] as usize +1];
		let mut impossible: bool = false;
		row_idx = 0;
		for j in 0..values[2]+1 {
			row_interval[j as usize][0] = if j == 0 {0} else {row_interval[j as usize - 1][1]+1};
			row_idx = row_interval[j as usize][0];
			row_interval[j as usize][1] = loop {
				if row_interval[j as usize][0] == values[0] as usize - 1 {
					break values[0] as usize - 1;
				};
				if hori_arr[row_idx] == (j+1) * choc_per_row {
					break row_idx;
				}
				row_idx += 1;
				if hori_arr[row_idx] > (j+1) * choc_per_row {
					println!("Case #{}: IMPOSSIBLE", i);
					impossible = true;
					break row_idx;
				}
			};
		}
		if impossible {
			continue;
		}

		col_idx = 0;
		for j in 0..values[3]+1 {
			col_interval[j as usize][0] = if j == 0 {0} else {col_interval[j as usize - 1][1]+1};
			col_idx = col_interval[j as usize][0];
			col_interval[j as usize][1] = loop {
				if col_interval[j as usize][0] == values[1] as usize -1 {
					break values[1] as usize - 1;
				}
				if vert_arr[col_idx] == (j+1) * choc_per_col {
					break col_idx;
				}
				col_idx += 1;
				if vert_arr[col_idx] > (j+1) * choc_per_col {
					println!("Case #{}: IMPOSSIBLE", i);
					impossible = true;
					break col_idx;
				}
			};
		}
		if impossible {
			continue;
		}
		// print!("row_interval: {:?}", row_interval);
		// print!("col_interval: {:?}", col_interval);
		
		
	}
}