#include <iostream>
#include <string>
#include <cstring>

int main() {
	int T;
	scanf("%d",&T);
	for (int i = 0; i < T; i++) {
		char input[1000];
		scanf("%s", input);

		int K;
		scanf("%d", &K);

		int len = (unsigned)strlen(input);

		int arr[len];
		for (int j = 0; j < len; j++) {
			arr[j] = 0;
		}

		int flips = 0;
		int max_flips = 0;
		for (int j = 0; j < len-K+1; j++) {
			flips += arr[j];
			if (input[j] == '-' && flips%2==0) {
				flips++;
				max_flips++;
				arr[j+K]--;
			}
			if (input[j] == '+' && flips%2==1) {
				flips++;
				max_flips++;
				arr[j+K]--;
			}
			// printf("%4d", flips);
		}
		// printf("\n");
		
		// for (int j = 0; j < len; j++) {
		// 	printf("%4d", arr[j]);
		// }

		int success = 1;
		int flips_back = flips;
		for (int j = len-K+1; j < len; j++) {
			flips += arr[j];
			if (input[j] == '-' && flips%2==0) {
				success = 0;
				break;
			}
			if (input[j] == '+' && flips%2==1) {
				success = 0;
				break;
			}
		}
		if (success) {
			printf("Case #%d: %d\n", i+1, max_flips);
		}
		else {
			printf("Case #%d: IMPOSSIBLE\n", i+1);
		}
	}
}