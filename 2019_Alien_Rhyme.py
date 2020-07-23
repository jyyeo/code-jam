import numpy as np

T = int(input())

for i in range(T):
	N = int(input());
	words = list()
	for j in range(N):
		new_str = input()
		words.append(new_str[::-1])
	words.sort()

	# # TESTING
	# print("\n")
	# for j in range(N):
	# 	print("{}".format(words[j]))
	# print("\n")

	# find substrings
	substr = [["" for i in range(N)] for j in range(N)]
	substr_length = [[0 for i in range(N)] for j in range(N)]
	for j in range(N):
		for k in range(N):
			# print("{} {} {} {}".format(words[j],words[k], j, k))
			if j == k:
				continue
			substr_length[j][k] = max([i for i in range(min(len(words[j]),len(words[k]))+1) if words[j][0:i] == words[k][0:i]])
			substr[j][k] = words[j][0:substr_length[j][k]]

	# remove pairs by substring length
	pairs = 0
	while(np.count_nonzero(substr_length)):
		m = max(substr_length)
		r = substr_length.index(max(substr_length))
		c = substr_length[r].index(max(substr_length[r]))

		substr_length[r] = [0 for i in range(N)]
		substr_length[c] = [0 for i in range(N)]
		for x in substr_length: x[r] = 0
		for x in substr_length: x[c] = 0

		delete_str = substr[r][c]
		for j in range(N):
			for k in range(N):
				if (substr[j][k] == delete_str):
					substr[j][k] = ''
					substr_length[j][k] = 0
		pairs = pairs + 2

	print("Case #{}: {}".format(i+1, pairs))


