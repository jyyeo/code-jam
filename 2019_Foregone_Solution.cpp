#include <iostream> 
#include <math.h>
using namespace std;

int findB(int N);

int main() {
    int T;
    cin >> T;
    for (int i = 0; i < T; i++) {
        int N, A, B;
        cin >> N;
        B = findB(N);
        A = N - B;
        cout << "Case #" << i+1 << ": " << A << " " << B << endl;
    }
    return 0;
}

int findB(int N) {
    int B = 0;
    int e = 0;
    while (N > 0) {
        if (N%10 == 4) {
            B += 3 * pow(10, e);
        }
        e++;
        N /= 10;
    }
    return B;
}