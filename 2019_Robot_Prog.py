T = int(input())

for i in range(T):
	A = int(input())
	strategies = list()
	for j in range(A):
		strategies.append(input())
	output = ""
	while (len(output) < 500):
		index = len(output)
		col  = [row[index%len(row)] for row in strategies]
		
		# all R, S, P appear in the same turn
		if (any(ele == 'R' for ele in col) and
			any(ele == 'S' for ele in col) and
			any(ele == 'P' for ele in col) ):
			output = "IMPOSSIBLE"
			break

		if (any(ele == 'R' for ele in col)):
			if (any(ele == 'S' for ele in col)):
				if (any(ele == 'P' for ele in col)):
					# R, S, P in the same turn
					output = "IMPOSSIBLE"
					break
				else:
					# R and S in the same turn
					output += 'R'
					remove_index = [i for i in range(len(col)) if col[i] == 'S']
					strategies = [strategies[i] for i in range(len(strategies)) if i not in remove_index]
			else:
				# R only
				output += 'P'
				break
		else:
			if (any(ele == 'S' for ele in col)):
				if (any(ele == 'P' for ele in col)):
					# S and P in the same turn
					output += 'S'
					remove_index = [i for i in range(len(col)) if col[i] == 'P']
					strategies = [strategies[i] for i in range(len(strategies)) if i not in remove_index]
				else:
					# S only
					output += 'R'
					break
			else:
				if (any(ele == 'P' for ele in col)):
					# P only
					output += 'S'
					break

	# print output
	print("Case #{}: {}".format(i+1, output))
