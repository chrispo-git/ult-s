from math import sqrt
import os

# Get vertex coordinates.
# Clockwise order of vertices is recommended over counterclockwise order.
vtx_x1 = float(input("Vertex 1 X Coordinate: "))
vtx_y1 = float(input("Vertex 1 Y Coordinate: "))
vtx_x2 = float(input("Vertex 2 X Coordinate: "))
vtx_y2 = float(input("Vertex 2 Y Coordinate: "))

# Calculate the directional vector from the vertex coordinates.
delta_x = vtx_x2 - vtx_x1
delta_y = vtx_y2 - vtx_y1

# Calculate the unit vector.
magnitude = sqrt(delta_x * delta_x + delta_y * delta_y)
norm_x = -delta_y / magnitude
norm_y = delta_x / magnitude

# Correct negative zero.
if norm_x == -0.0:
    norm_x = 0.0

# Print the resulting normal.
print(f'{norm_x}, {norm_y}')
os.system("pause")