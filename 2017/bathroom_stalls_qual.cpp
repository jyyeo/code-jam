#include <iostream>
#include <vector>
#include <algorithm>

int main() {
	int T;
	scanf("%d", &T);

	for (int i = 1; i <= T; i++) {
		int N, K;
		scanf("%d %d", &N, &K);
		
		std::vector<long long> size;
		size.push_back(N);
		long long left_half; //min
		long long right_half; // max
		long long max_range;
		long long max_idx;
		for (int j = 0; j < K; j++) {

			max_range = *std::max_element(size.begin(), size.end());
			max_idx = std::max_element(size.begin(), size.end()) - size.begin();
			// printf("%4lld %4lld\n", max_range, max_idx);
			left_half = (max_range%2==0) ? max_range/2 - 1 : max_range/2;
			right_half = max_range/2;

			if (j < K-1) {
				size.erase(size.begin() + max_idx);
				size.push_back(left_half);
				size.push_back(right_half);
			}
		}
		printf("Case #%d: %lld %lld\n", i, right_half, left_half);
	}
}