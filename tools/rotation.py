import numpy
import math

def deg_to_radian(deg):
    return deg * 3.14159265359 / 180.0

def xrotate(a):
    return numpy.array([
     [1,0,0],
     [0,math.cos(a),math.sin(a)],
     [0,-math.sin(a),math.cos(a)]
    ])

def yrotate(b):
    return numpy.array([
     [math.cos(b), 0, -math.sin(b)],
     [0, 1, 0],
     [math.sin(b), 0, math.cos(b)]
    ])

def zrotate(c):
    return numpy.array([
     [math.cos(c), -math.sin(c), 0],
     [math.sin(c), math.cos(c), 0],
     [0, 0, 1]
    ])

mat = numpy.matmul(
    yrotate(deg_to_radian(45)),
    xrotate(deg_to_radian(-35.264389701728)),
)

# mat = xrotate(deg_to_radian(-35.264389701728))
for x in range(3):
    print("[", end="")
    for y in range(3):
        print("{}0, ".format(mat[x][y]), end="")
    print("0.0],")
