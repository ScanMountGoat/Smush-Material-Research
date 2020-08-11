import sys
import os
import matplotlib.pyplot as plt 
import numpy as np 
import pandas as pd

def row_to_str(row):
    # Combine columns back into a CSV format in case there are multiple values in a struct.
    col_values = []
    for x in row.values:
        if type(x) is np.float64:
            col_values.append(str(round(x,3)))
        else:
            col_values.append(str(x))
    return ','.join(col_values)

if len(sys.argv) < 2:
    print(f'Usage: {sys.argv[0]} <value_occurrences.csv>')
    exit(1)
    
# TODO: Add an option for SQLite
# The top n can be replaced with connecton.FetchMany(n)
df = pd.read_csv(sys.argv[1])

# Only graph the ten most common values from most to least common.
size = min(df.shape[1],10)
values = df.drop('Occurrences',axis=1)
values = values.apply(row_to_str, axis=1).values[:size][::-1]
counts = df['Occurrences'].values[:size][::-1]

# Use an adjusted horizontal plot to avoid cutting off the value labels.
plt.gcf().subplots_adjust(left=0.25)

plt.title(os.path.basename(sys.argv[1]))
plt.xlabel('Number of occurrences')
plt.autoscale()
plt.barh(np.arange(values.size),counts,tick_label=values)
plt.show()