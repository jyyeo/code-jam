#include <iostream>
#include <algorithm>

using namespace std;

int *find_primes(int N) {
	bool prime[N+1];
	memset(prime, true, N+1);

	for (int p = 2; p*p <= N; p++) {
		if (prime[p] == true) {
			for (int i = p*2; i <= N; i += p) {
				prime[i] = false;
			}
		}
	}
	int mycount = count(prime, prime + N, true) - 2;
	int all_primes[mycount];
	int idx = 0;
	for (int p = 2; p <= N; p++){
		if (prime[p]) {
			all_primes[idx] = prime[p];
			idx++;
		}
	}
	return all_primes;
}

// int common_factor(int a, int b) {
// 	int 
// }

int main() {
	int T;
	cin >> T;
	for (int i = 0; i < T; i++) {
		int N, L;
		cin >> N >> L;
		int *all_primes;
		all_primes = find_primes(N);
		int ciphertext[L];
		for (int j = 0; j < L; j++) {
			int s;
			cin >> s;
			ciphertext[j] = s;
		}
		int primes[L+1];
		// for (int j = 0; j < L-1; j++) {
		// 	primes[j] = common_factor(ciphertext[j], ciphertext[j+1]);
		// }

		// sort(ciphertext, ciphertext + L);
		// for (int j = 0; j < L; j++) {
		// 	cout << ciphertext[j] << " ";
		// }
	}
	return 0;
}