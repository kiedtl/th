#!/usr/bin/env python3
#
# maze-mg.py: a prototype of an experimental
# mapgen algorithm based on rooms and mazes
#
# (c) 2020 Kiëd Llaentenn
# See the COPYING file for license info.

import random

floor = ' '
wall  = '▒'
start = '@'
end   = '*'

width = 101
height = 51

def display(m):
    for y in m:
        for x in y:
            print(x, end='')
        print('')

def genmaze(m, cx, cy):
    neighbors = [((-1, 0), (-2, 0)),
                 ((0,  -1), (0,  -2)), ((0,  1), (0,  2)),
                 ((1,  0), (2,  0))]
    random.shuffle(neighbors)
    for neighbor in neighbors:
        (iy, ix) = (cy + neighbor[0][0], cx + neighbor[0][1])
        (ny, nx) = (cy + neighbor[1][0], cx + neighbor[1][1])
        if nx > 0 and ny > 0 and nx < (width - 1) and ny < (height - 1) and m[ny][nx] == wall:
            m[ny][nx] = floor
            m[iy][ix] = floor
            e = genmaze(m, nx, ny)

def remove_deadends(m, goal):
    for _ in range(0, goal):
        found_dead_ends = 0
        for y in range(0, height - 1):
            for x in range(0, width - 1):
                neighbors = [(-1, 0), (0,  -1), (0,  1), (1,  0)]
                neighbor_walls = 0
                for neighbor in neighbors:
                    (ny, nx) = (y + neighbor[0], x + neighbor[1])
                    if nx > 0 and ny > 0 and nx < (width - 1) and ny < (height - 1):
                        if m[ny][nx] == wall:
                            neighbor_walls += 1
                if neighbor_walls >= 3:
                    m[y][x] = wall
                    found_dead_ends += 1

# create matrix
matrix = [[wall for x in range(0, width)] for y in range(0, height)]

# set start position
startpos = (1, 1)
matrix[startpos[0]][startpos[1]] = start

# generate maze
genmaze(matrix, startpos[0], startpos[1])
remove_deadends(matrix, 20)

# display matrix
display(matrix)
