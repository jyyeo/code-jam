#include <iostream>

int main() {
	int T;
	scanf("%d", &T);

	for (int i = 1; i <= T; i++) {
		double D;
		int N;
		scanf("%lf %d", &D, &N);
		double time = 0.0, time_curr = 0.0;
		double max_speed = 1e99, max_speed_curr = 1e99;
		for (int j = 0; j < N; j++) {
			double distance, speed;
			scanf("%lf %lf", &distance, &speed);
			max_speed_curr = D * speed / (D - distance);
			if (max_speed_curr < max_speed) {
				max_speed = max_speed_curr;
			}
		}
		printf("Case #%d: %lf\n", i, max_speed);
	}
}