import sys
import os
import matplotlib.pyplot as plt 
import numpy as np 

if len(sys.argv) < 2:
    print(f'Usage: {sys.argv[0]} <value occurrences dump file>')
    exit(1)
    
values = []
counts = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        sections = line.split(' ')
        values.append(sections[0])
        counts.append(int(sections[1]))
size = min(len(values),10)
values = values[::-1][-size:]
counts = counts[::-1][-size:]

# Display larger bars on top.
values = np.array(values)
counts = np.array(counts)

# Use an adjusted horizontal plot to avoid cutting off the value labels.
plt.gcf().subplots_adjust(left=0.25)
plt.title(os.path.basename(sys.argv[1]))
plt.autoscale()
plt.barh(np.arange(values.size),counts,tick_label=values)
plt.show()