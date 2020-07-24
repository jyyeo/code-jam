T = int(input())

for i in range(T):
	N, K = [int(s) for s in input().split(" ")]
	charles_skill = [int(s) for s in input().split(" ")]
	delila_skill = [int(s) for s in input().split(" ")]

	pairs = 0
	for j in range(N):
		start = j
		end = j+1

		while (end <= N):
			max_charles = max(charles_skill[start:end])
			max_delila = max(delila_skill[start:end])
			# print(start, end, max_charles, max_delila)
			if (abs(max_charles - max_delila) <= K):
				pairs = pairs + 1 
				# print(start,end)

			end = end + 1
			
	print("Case # {}: {}".format(i+1, pairs))		
	# print(pairs)
