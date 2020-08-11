import sys
import os
import matplotlib.pyplot as plt 
from mpl_toolkits.mplot3d import Axes3D
import numpy as np 
import sqlite3
import pandas as pd

select_param_id_sql = 'SELECT Id FROM CustomParam WHERE Name = ?'
select_values_sql = """
SELECT Value1, Value2, Value3, Value4, COUNT(*) AS Count 
FROM CustomVectorParam
WHERE ParamID = ?
GROUP BY Value1, Value2, Value3, Value4
"""


def get_values_csv(csv_file):
    df = pd.read_csv(csv_file)
    # Scale integers to a more suitable range for RGB display.
    result = np.float32(df.values)
    if df.dtypes[0] == np.int64 or df.dtypes[0] == np.int32:
        print('Normalized integers to floats.')
        print('[0,255] -> [0.0,1.0]')
        result[:,:3] = result[:,:3] / 255
    # Get top results to make the display less dense.
    count = 5000
    print(f'Selected top {count} results.')
    return result[:count,:]


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
    if len(sys.argv) == 3:
        db_file = sys.argv[1]
        param_name = sys.argv[2]
        values = get_values_sqlite(db_file, param_name)
    elif len(sys.argv) == 2:
        csv_file = sys.argv[1]
        param_name = os.path.basename(csv_file)
        values = get_values_csv(csv_file)
    else:
        print('Usage:')
        print(f'\t{sys.argv[0]} <Smush Materials SQLite DB> <CustomVector Param>')
        print(f'\t{sys.argv[0]} <Value Occurrences.csv>')
        exit(1)

    graph_values(param_name, values)

