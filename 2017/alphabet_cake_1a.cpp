#include <iostream>
#include <vector>

#define SIZE 30

void populate_left(char grid[][SIZE], int row, int col) {
	if (col != 0 && grid[row][col-1] == '?') {
		grid[row][col-1] = grid[row][col];
		populate_left(grid, row, col-1);
	}
}

void populate_right(char grid[][SIZE], int row, int col, int C) {
	if (col+1 < C && grid[row][col+1] == '?') {
		grid[row][col+1] = grid[row][col];
		populate_left(grid, row, col+1);
	}
}

int main() {
	int T;
	scanf("%d", &T);

	for (int i = 1; i <= T; i++) {
		int R, C;
		scanf("%d %d", &R, &C);

		char grid[SIZE][SIZE];
		for (int j = 0; j < R; j++) {
			scanf("%s", grid[j]);
		}

		for (int j = 0; j < R; j++) {
			for (int k = 0; k < C; k++) {
				if (grid[j][k] != '?') {
					populate_left(grid, j, k);
					populate_right(grid, j, k, C);
				}
			}
		}

		for (int j = 1; j < R; j++) {
			for (int k = 0; k < C; k++) {
				if (grid[j][k] == '?') {
					grid[j][k] = grid[j-1][k];
				}
			}
		}

		for (int j = R-2; j >= 0; j--) {
			for (int k = 0; k < C; k++) {
				if (grid[j][k] == '?') {
					grid[j][k] = grid[j+1][k];
				}
			}
		}

		printf("Case #%d:\n", i);
		for (int j = 0; j < R; j++) {
			printf("%s\n", grid[j]);
		}
	}
}