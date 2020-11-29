import numpy as np 

def get_word(word, length, L,transpose, joined, i):
	if length == L:
		word = ''.join(word)
		if word in joined:
			return 0
		else:
			print("Case #{}: {}".format(i+1, word))
			return 1
	
	for j in range(N):
		word[length] = transpose[length][j]
		if get_word(word, length+1, L, transpose, joined, i) == 1:
			return 1
	return 0


T = int(input())

for i in range(T):
	N, L = [int(s) for s in input().split(" ")]
	arr = [['']*L]*N
	joined = [""]*N
	
	for j in range(N):
		arr[j] = list(input())
		joined[j] = ''.join(arr[j])

	# print(arr)
	# print(joined)

	transpose = np.transpose(arr)
	
	word = ['']*L
	# print("i=",i)
	if get_word(word, 0, L, transpose, joined, i) == 0:
		print("Case #{}: -".format(i+1))

	
