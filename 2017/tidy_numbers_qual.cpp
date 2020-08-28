#include <iostream>
#include <cstring>
#include <math.h>

int main() {
	int T;
	scanf("%d", &T);

	for (int i = 1; i <= T; i++) {
		char input[18];
		scanf("%s", input);
		int len = (unsigned)strlen(input);

		// convert to an array of integers
		int N[18];
		for (int j = 0; j < len; j++) {
			N[j] = (int)input[j] - '0';
		}

		int output[18] = {};
		int inversion = len;
		if (len == 1) {
			output[0] = N[0];
		}
		for (int j = 0; j < len; j++) {
			output[j] = N[j];
			
			if (len == 1) break;

			if (N[j] > N[j+1] && j+1 < len) {
				inversion = -1;
				for (int k = 0; k < j; k++) {
					if (N[k] < N[k+1]) {
						inversion = k;
					}
				}
				if (inversion != len) { // inversion has been edited
					break;
				}
			}
		}

		// for (int j = 0; j < len; j++) {
		// 	printf("%d", output[j]);
		// } printf("\n");

		// printf("inversion %d\n", inversion);

		if (inversion == len) {
			printf("Case #%d: ", i);
			for (int j = 0; j < len; j++) {
				printf("%d", output[j]);
			}
			printf("\n");
		}

		else {
			printf("Case #%d: ", i);
			for (int j = 0; j < len; j++) {
				if (j == 0 && inversion == -1 && N[0] == 1) {
					continue; // don't print leading 0
				}

				else if (j == inversion+1) {
					output[j] = N[j] - 1;
					// printf("%d", output[j]);
				}

				else if (j > inversion+1) {
					output[j] = 9;
					
				}
				printf("%d", output[j]);
			}
			printf("\n");
		}
	}
}