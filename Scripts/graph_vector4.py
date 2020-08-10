import sys
import os
import matplotlib.pyplot as plt 
from mpl_toolkits.mplot3d import Axes3D
import numpy as np 
import sqlite3

select_param_id_sql = 'SELECT Id FROM CustomParam WHERE Name = ?'
select_values_sql = 'SELECT DISTINCT Value1, Value2, Value3, Value4 FROM CustomVectorParam WHERE ParamID = ?'

def get_values(db_file, param_name):
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

    # Plot the values in 3d.
    fig = plt.figure()
    ax = Axes3D(fig)

    # Chart title
    ax.text2D(0.05, 0.95, param_name, transform=ax.transAxes) 

    ax.set_xlabel('Red (X)')
    ax.set_ylabel('Green (Y)')
    ax.set_zlabel('Blue (Z)')
    ax.scatter(values[:,0],values[:,1],values[:,2],c=colors,edgecolor='k') 

    plt.show()


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print('Usage:')
        print(f'\t{sys.argv[0]} <Smush Materials SQLite DB> <CustomVector Param>')
        exit(1)
        
    db_file = sys.argv[1]
    param_name = sys.argv[2]

    values = get_values(db_file, param_name)
    graph_values(param_name, values)

