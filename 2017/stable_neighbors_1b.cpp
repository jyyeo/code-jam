#include <iostream>

char character_of (int color_int) {
	if (color_int == 1) return 'R';
	if (color_int == 2) return 'O';
	if (color_int == 3) return 'Y';
	if (color_int == 4) return 'G';
	if (color_int == 5) return 'B';
	else return 'V';
}

int number_of (char color) {
	if (color == 'R') return 1;
	if (color == 'O') return 2;
	if (color == 'Y') return 3;
	if (color == 'G') return 4;
	if (color == 'B') return 5;
	else return 6;
}

char form_chain (int input[7], char sec_color) {
	if (sec_color == 'O') { return 'B'; }
	else if (sec_color == 'G') { return 'R'; }
	else { return 'Y'; }
}

char find_max (int input[7], char color1, char color2) {
	int quantity1 = input[number_of(color1)];
	int quantity2 = input[number_of(color2)]; 
	if (quantity1 == 0 && quantity2 == 0) {
		return 'I';
	}

	int max_quantity;
	max_quantity = (quantity1 > quantity2) ? quantity1 : quantity2;
	if (max_quantity == quantity1) return color1;
	else return color2;
	
}

char find_next_color (int input[7], char curr_color) {
	char next_color;
	if (curr_color == 'R') {
		if (input[4] != 0) {
			next_color = 'G';
		}
		else {
			next_color = find_max(input, 'Y', 'B');
		}
	}
	if (curr_color == 'O') {
		next_color = (input[5] == 0) ? 'I' : 'B'; 
	}
	if (curr_color == 'Y') {
		if (input[6] != 0) {
			next_color = 'v';
		}
		else {
			next_color = find_max(input, 'R', 'B');
		}
	}
	if (curr_color == 'G') {
		next_color = (input[1] == 0) ? 'I' : 'R'; 
	}
	if (curr_color == 'B') {
		if (input[2] != 0) {
			next_color = 'O';
		}
		else {
			next_color = find_max(input, 'R', 'Y');
		}
	}
	if (curr_color == 'V') {
		next_color = (input[3] == 0) ? 'I' : 'Y';
	}
	return next_color;
}

bool clash (char color1, char color2) {
	if (color1 == 'R') {
		if (color2 == 'Y' || color2 == 'G' || color2 == 'B') {
			return false;
		}
	}
	if (color1 == 'O' && color2 == 'B') {
		return false;
	}
	if (color1 == 'Y') {
		if (color2 == 'R' || color2 == 'B' || color2 == 'V') {
			return false;
		}
	}
	if (color1 == 'G' && color2 == 'R') {
		return false;
	}
	if (color1 == 'B') {
		if (color2 == 'R' || color2 == 'O' || color2 == 'Y') {
			return false;
		}
	}
	if (color1 == 'V' && color2 == 'Y') {
		return false;
	}
	return true;
}

int main() {
	int T;
	scanf("%d", &T);

	for (int i = 1; i <= T; i++) {
		int fail = 0;
		int N, R, O, Y, G, B, V;
		int input[7];
		scanf("%d %d %d %d %d %d %d", &N, &R, &O, &Y, &G, &B, &V);
		input[0] = N;
		input[1] = R;
		input[2] = O;
		input[3] = Y;
		input[4] = G;
		input[5] = B;
		input[6] = V;

		char output[1000]; // output
		char curr_color, next_color;
		for (int j = 0; j < N; j++) {

			// put O, G, V horses in first
			int idx = 2;
			while (input[idx] == 0 && idx < 7) {
				idx += 2;
			}
			// create chain with O/G/V horse in between
			if (idx < 7) {
				char pri_color = form_chain(input, character_of(idx));

				if ((N-1)-j > 2) {
					output[j] = pri_color;
					output[j+1] = character_of(idx);
					output[j+2] = pri_color;
					j += 2;
					//update remaining horses
					input[number_of(pri_color)] -= 2;
					input[idx]--;
					if (input[number_of(pri_color)] < 0) fail = 1;
					// update curr color
					curr_color = pri_color;
				}
				else if ((N-1)-j > 1) {
					output[j] = pri_color;
					output[j+1] = character_of(idx);
					j++;
					//update remaining horses
					input[number_of(pri_color)]--;
					input[idx]--;
					if (input[number_of(pri_color)] < 0) fail = 1;
					// update curr color
					curr_color = character_of(idx);
				}
				else {
					output[j] = character_of(idx);
					//update remaining horses
					input[idx]--;
					// update curr color
					curr_color = character_of(idx);
				}
			}

			else {
				if (j == 0) {
					idx = 1;
					while (input[idx] == 0) {
						idx += 2;
					}
					output[j] = character_of(idx);
					input[idx]--;
				}
				else {
					output[j] = next_color;
					input[number_of(next_color)]--;
				}
				curr_color = output[j];
			}

			if (j < N-1) {
				next_color = find_next_color(input, curr_color);
				if (next_color == 'I') {
					printf("Case #%d: IMPOSSIBLE\n", i);
					fail = 1;
					break;
				}
			}
			// printf("output[j] = %c next_color = %c\n", output[j], next_color);
		}
		if (!fail) {
			if (clash(output[0], output[N-1])) { // last clash with first
				printf("Case #%d: IMPOSSIBLE\n", i);
			}
			else {
				printf("Case #%d: ", i);
				for (int j = 0; j < N; j++) {
					printf("%c", output[j]);
				}
				printf("\n");
			}
		}
	}
}