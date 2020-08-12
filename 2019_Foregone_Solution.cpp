#include <iostream> 
#include <string>
#include <math.h>

using namespace std;

int main() {
    int T;
    cin >> T;
    for (int i = 0; i < T; i++) {
        string N, A, B;
        int c = 0;
        cin >> N;
        while (c < N.length()) {
            if (N[c] == '4') {
                A += '1';
                B += '3';
            }
            else {
                A += N[c];
                if (!B.empty()) {
                    B += '0';
                }
            }
            c++;
        }
        cout << "Case #" << i+1 << ": " << A << " " << B << endl;
    }
    return 0;
}
