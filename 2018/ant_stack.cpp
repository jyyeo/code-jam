#include <iostream> 
#include <vector>
#include <sstream>
#include <stdlib.h>

int main() {
	int T;
	std::cin >> T;

	for (int i = 0; i < T; i++) {
		int N;
		std::cin >> N;
		std::ws(std::cin);
		std::vector<int> input(N);
		std::string line;
		for (int j = 0; j < N; j++) {
			scanf("%d", &input[j]);
		}

		int cumsum[10000][150];
		for (int j = 0; j < 10000; j++) {
			for (int k = 0; k < 150; k++) {
				cumsum[j][k] = -1;
			}
		}
		// cumsum[i][j] is the max weight given j number of elements in stack, 
		// by the i-th element
		cumsum[0][0] = 0;
		for (int j = 1; j <= N; j++) {
			for (int k = 0; k <= 150; k++) {
				if (k == 0) {
					cumsum[j][k] = 0;
					continue;
				}

				if (cumsum[j-1][k-1] == -1) continue;

				cumsum[j][k] = cumsum[j-1][k];
				if (input[j-1] * 6 >= cumsum[j-1][k-1]) {
					if (cumsum[j][k] == -1) {
						cumsum[j][k] = cumsum[j-1][k-1] + input[j-1];
					} else {
						cumsum[j][k] = std::min(cumsum[j-1][k-1] + input[j-1], cumsum[j][k]);
					}
				} 
			}
			// for (int k = 0; k <= N; k++) {
			// 	printf("%d ", cumsum[j][k]);
			// } printf("\n");
		}

		int max = 1;
		for (int j = 0; j <= N; j++) {
			if (cumsum[N][j] != -1) {
				max = j;
			} else {
				break;
			}
		}
		printf("Case #%d: %d\n", i+1, max);
	}
}