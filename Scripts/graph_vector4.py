import sys
import os
import matplotlib.pyplot as plt 
from mpl_toolkits.mplot3d import Axes3D
import numpy as np 
import ast

if len(sys.argv) < 2:
    print('Usage:')
    print(f'\t{sys.argv[0]} <unique value dump file>')
    exit(1)
    
values = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        # Workaround for tuples having spaces between values
        # Vectors will be read as tuples of floats
        value = line[line.index(' ')+1:line.index('\\')]
        values.append(ast.literal_eval(value))
values = np.array(values)

# Use the XYZ values as colors.
colors = np.clip(values[:,:3],0,1)

# Plot the values in 3d.
fig = plt.figure()
ax = Axes3D(fig)

# Chart title
ax.text2D(0.05, 0.95, os.path.basename(sys.argv[1]), transform=ax.transAxes) 

# Ignore colors that are too bright.
# ax.set_xlim(0, 1)
# ax.set_ylim(0, 1)
# ax.set_zlim(0, 1)
ax.set_xlabel('Red')
ax.set_ylabel('Green')
ax.set_zlabel('Blue')
ax.scatter(values[:,0],values[:,1],values[:,2],c=colors,edgecolor='k') 

plt.show()