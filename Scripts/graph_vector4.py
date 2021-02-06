import sys
import os
import matplotlib.pyplot as plt 
from mpl_toolkits.mplot3d import Axes3D
import numpy as np 
import sqlite3
import plotly.express as px
import plotly.graph_objs as go

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
    # Size based on number of occurrences mapped to a more reasonable range.
    sizes = (values[:,4] / np.max(values[:,4])) * 30 + 10

    trace = go.Scatter3d(x=values[:,0],
                        y=values[:,1],
                        z=values[:,2],
                        customdata = values[:,4],
                        hovertemplate='<b>x:%{x}</b><br><b>y:%{y}<b><br><b>z: %{z}<b><br><b>occurrences: %{customdata}</b><extra></extra>',
                        mode='markers',
                        marker=dict(line=dict(width=2,color='DarkSlateGray',),size=sizes,
                                    color=['rgb({},{},{})'.format(r,g,b) for r,g,b in np.clip((values[:,:3]*255).astype(np.int32),0,255)],opacity=0.9))

    data = [trace]

    layout = go.Layout(margin=dict(l=0,r=0,b=0,t=0))

    fig = go.Figure(data=data, layout=layout)
    fig.show()


if __name__ == '__main__':  
    if len(sys.argv) != 3:
        print('Usage:')
        print(f'\tpython {sys.argv[0]} <Smush Materials SQLite DB> <CustomVector Param>')
        exit(1)

    db_file = sys.argv[1]
    param_name = sys.argv[2]
    values = get_values_sqlite(db_file, param_name)

    graph_values(param_name, values)

