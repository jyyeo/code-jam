import numpy as np

def maze_solution(i, j, maze, R, C, step, end):

    # maze is complete
    if (np.count_nonzero(maze) == R*C):
        end = 1
        return end, maze

    else:        
        range_R = [*range(R)]
        range_C = [*range(C)]
        if (step != 1):
            range_R.remove(i)
            range_C.remove(j)

        for a in range_R:
            for b in range_C:
            	if (abs(a-i) != abs(b-j) and (i+j) != (a+b)):
	                if (maze[a][b] == 0):
	                    maze_copy = maze
	                    maze_copy[a][b] = step
	                    end, maze_temp = maze_solution(a, b, maze_copy, R, C, step+1, end)
        
        if (end != 1):
            r, c = np.where(maze == step-1)
            maze[r,c] = 0
        
    return end, maze

T = int(input())
for j in range(T):
    R, C = [int(s) for s in input().split(" ")]
    empty_maze = np.zeros([R,C])
    step = 1 
    end = 0
    end, maze = maze_solution(0, 0, empty_maze, R, C, step, end)

    # maze is possible
    if (np.count_nonzero(maze) == R*C):
        print("Case #{}: POSSIBLE".format(T))
        for i in range(1, R*C+1):
            r, c = np.where(maze == i)
            print("{} {}". format(int(r+1), int(c+1)))

    # maze is impossible
    else:
        print("Case #{}: IMPOSSIBLE".format(j+1))