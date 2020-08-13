def generate_output(N):
	test = []
	for j in range(N):
		if (j < N/2):
			test += '0'
		else:
			test += '1'
	return test


def update_verdict(verdict, response, num_lines, N):
	while (num_lines > 0):
		print(num_lines)
		left = update_verdict(verdict[:N//2-1], response, num_lines-1, N//2)
		right = update_verdict(verdict[N//2:], response, num_lines-1, N-N//2)
		verdict = left + right
	zeros = response.count('0')
	ones = response.count('1')
	if (zeros < N//2 and ones == N - N//2): # only left half
		verdict[N//2:] = 1
	if (zeros == N/2 and ones < N - N//2): # only right half
		verdict[:N//2-1] = 1
	print(verdict)
	return verdict

T = int(input())

for i in range(T):
	N, B, F = [int(s) for s in input().split(" ")]
	num_lines = 0
	verdict = [0] * N
	while (num_lines < F):
		if (num_lines == 0):
			test = generate_output(N)
		else:
			response = input()
			update_verdict(verdict, response, num_lines, N)
			# print (verdict.count(0))
			# if (verdict.count('0') == B):
			# 	idx = [chr(j) for j in range(N) if verdict[j] == 0]
			# 	output_string = ""
			# 	output_string.join(idx)
			# 	print(output_string, flush = True)
			# else:
			# 	verdict = update_verdict(verdict, num_lines, N)

		print("verdict: " + str(verdict))
		print("TEST_STORE %s", test, flush = True)
		num_lines = num_lines + 1
		prev_test = test