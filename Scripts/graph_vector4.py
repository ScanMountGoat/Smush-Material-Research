import sys
import os
import matplotlib.pyplot as plt 
from mpl_toolkits.mplot3d import Axes3D
import numpy as np 
import sqlite3
import pandas as pd

select_param_id_sql = 'SELECT Id FROM CustomParam WHERE Name = ?'
select_values_sql = """
SELECT X, Y, Z, W, COUNT(*) AS Count 
FROM CustomVectorParam
WHERE ParamID = ?
GROUP BY X, Y, Z, W
"""

def get_values_sqlite(db_file, param_name):
    with sqlite3.connect(db_file) as con:
        param_id = con.execute(select_param_id_sql, (param_name,)).fetchone()
        if param_id is None:
            print(f'Could not find the Param ID for {param_name}')
            exit(1)

        values = con.execute(select_values_sql, param_id).fetchall()
        return np.array(values)


def graph_values(param_name, values):
    # Use the XYZ values as colors.
    colors = np.clip(values[:,:3],0,1)

    # Size based on number of occurrences mapped to a more reasonable range.
    sizes = (values[:,4] / np.max(values[:,4])) * 400 + 20

    # Plot the values in 3d.
    fig = plt.figure()
    ax = Axes3D(fig)

    # Chart title
    ax.text2D(0.05, 0.90, f'{param_name}\n(size = occurrences)', transform=ax.transAxes) 

    ax.set_xlabel('Red (X)')
    ax.set_ylabel('Green (Y)')
    ax.set_zlabel('Blue (Z)')
    ax.scatter(values[:,0],values[:,1],values[:,2],c=colors,s=sizes,edgecolor='k') 

    plt.show()


if __name__ == '__main__':  
    if len(sys.argv) != 3:
        print('Usage:')
        print(f'\tpython {sys.argv[0]} <Smush Materials SQLite DB> <CustomVector Param>')
        exit(1)

    db_file = sys.argv[1]
    param_name = sys.argv[2]
    values = get_values_sqlite(db_file, param_name)

    graph_values(param_name, values)

