# given a range, generate half 0, half 1
def generate_test(N):
	test = []
	for i in range(N//2):
		test += '0'
	for i in range(N-N//2):
		test += '1'
	return test

N = int(input())
test = generate_test(N)
print(test)
