T = int(input())

for i in range(T):
	cumsum = 0
	updated_cumsum = 0
	N, L = [int(s) for s in input().split(" ")]
	
	arr = list(map(int,input().split(" ")))
	unanswered = N - sum(arr)
	
	for j in range(L):

		rounded = round(arr[j] / N * 100)
		print(rounded)
		if rounded < arr[j] * 100:
			k = 1
			while k <= unanswered and unanswered > 0:
				# print("unanswered=", unanswered)
				if round((arr[j] + k) / N * 100) > (arr[j] + k) / N * 100:
					updated_cumsum += round((arr[j] + k) / N * 100)
					# print("k=",k)
					unanswered -= k
					break
				k += 1
		if updated_cumsum == cumsum: # no increment will increase % point
			updated_cumsum += round(arr[j] / N * 100)
		cumsum = updated_cumsum
		# print("cumsum=",cumsum)
	
	k = 1
	while k <= unanswered and unanswered > 0:
		# print("unanswered=",unanswered)
		if round(k / N * 100) > k / N * 100:
			cumsum += round(k / N * 100)
			unanswered -= k
			k = 1
			# print("1cumsum=", cumsum)
			continue
		elif k == unanswered:
			cumsum += round(1 / N * 100)
			k = 1
			unanswered -= 1  
		else:
			k += 1
		# print("cumsum=", cumsum)
	print("Case #{}: {}".format(i+1, cumsum))
