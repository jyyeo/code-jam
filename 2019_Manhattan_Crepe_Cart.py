import numpy as np 

def add_north(grid, r):
	for i in range(r+1, len(grid)):
		grid[i] = [elem+1 for elem in grid[i]]
	for i in range(r+1):
		grid[i] = [elem-1 for elem in grid[i]]
	return grid

def add_south(grid, r):
	for i in range(r):
		grid[i] = [elem+1 for elem in grid[i]]
	for i in range(r, len(grid)):
		grid[i] = [elem-1 for elem in grid[i]]
	return grid

def add_west(grid, c):
	for i in range(len(grid)):
		grid[i][:c] = [elem+1 for elem in grid[i][:c]]
		grid[i][c:] = [elem-1 for elem in grid[i][c:]]
	return grid

def add_east(grid, c):
	for i in range(len(grid)):
		grid[i][c+1:] = [elem+1 for elem in grid[i][c+1:]]
		grid[i][:c+1] = [elem-1 for elem in grid[i][:c+1]]
	return grid

def print_grid(grid):
	for i in range(len(grid)-1, -1, -1):
		print(grid[i])


T = int(input())

for i in range(T):
	P, Q = [int(s) for s in input().split(" ")]
	p = P + 1
	Q = Q + 1
	grid = [[0 for j in range(Q)] for k in range(Q)]
	for j in range(P):
		line = input().split(" ")
		c, r, d = [int(line[0]), int(line[1]), line[2]]
		if (d == 'N'):
			grid = add_north(grid, r)
		if (d == 'S'):
			grid = add_south(grid, r)
		if (d == 'W'):
			grid = add_west(grid, c)
		if (d == 'E'):
			grid = add_east(grid, c)
	max_val = np.amax(grid)
	indices = np.where(grid == max_val)
	list_of_coord = list(zip(indices[1], indices[0]))
	index = list_of_coord[0]

	print("Case #{}: {} {}".format(i+1, index[0], index[1]))
