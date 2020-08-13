#include <iostream> 
#include <string>

using namespace std;

int main() {
	int T;
	cin >> T;
	for (int i = 0; i < T; i++) {
		int N;
		cin >> N;
		string her_path, my_path;
		cin >> her_path;
		int k = 0;
		for (int j = 0; j < (2*N-2); j++) {
			if (her_path[k] == 'S') {
				my_path += 'E';
			}
			else {
				my_path += 'S';
			}
			k++;
		}
		cout << "Case #" << i+1 << ": " << my_path << endl;
	}
}