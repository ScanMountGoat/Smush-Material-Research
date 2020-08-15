import sys
import os
import sqlite3

select_vector_params_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, CustomParam.Name as ParamName
FROM CustomVectorParam
INNER JOIN Material
	ON CustomVectorParam.MaterialID = Material.ID
INNER JOIN CustomParam
	ON CustomVectorParam.ParamID = CustomParam.ID
WHERE Shader = ?"""

select_bool_params_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, CustomParam.Name as ParamName
FROM CustomBooleanParam
INNER JOIN Material
	ON CustomBooleanParam.MaterialID = Material.ID
INNER JOIN CustomParam
	ON CustomBooleanParam.ParamID = CustomParam.ID
WHERE Shader = ?"""

select_float_params_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, CustomParam.Name as ParamName
FROM CustomFloatParam
INNER JOIN Material
	ON CustomFloatParam.MaterialID = Material.ID
INNER JOIN CustomParam
	ON CustomFloatParam.ParamID = CustomParam.ID
WHERE Shader = ?"""

select_attributes_sql = """SELECT DISTINCT SUBSTR(ShaderLabel, 1, 24) as Shader, MeshAttribute
FROM ShaderMeshAttribute
WHERE Shader = ?"""

select_tags_sql = """SELECT DISTINCT SUBSTR(ShaderLabel, 25) as Tag
FROM Material
WHERE SUBSTR(Material.ShaderLabel, 1, 24) = ?"""


def get_values_sqlite(db_file, shader_name):
    with sqlite3.connect(db_file) as con:
        bool_params = con.execute(select_bool_params_sql, (shader_name,)).fetchall()
        float_params = con.execute(select_float_params_sql, (shader_name,)).fetchall()
        vector_params = con.execute(select_vector_params_sql, (shader_name,)).fetchall()
        attributes = con.execute(select_attributes_sql, (shader_name,)).fetchall()
        tags = con.execute(select_tags_sql, (shader_name,)).fetchall()

        return bool_params, float_params, vector_params, attributes, tags


def print_records(records, index=1):
    for record in records:
        print(record[index])


if __name__ == '__main__':  
    if len(sys.argv) != 3:
        print('Usage:')
        print(f'\tpython {sys.argv[0]} <Smush Materials SQLite DB> <Shader Label>')
        print(f'\tex: {sys.argv[0]} SmushMaterials.db SFX_PBS_0100000008018269')
        exit(1)

    db_file = sys.argv[1]
    shader_name = sys.argv[2]
    bool_params, float_params, vector_params, attributes, tags = get_values_sqlite(db_file, shader_name)

    print('Material Parameters:')
    print_records(bool_params)
    print_records(float_params)
    print_records(vector_params)
    print()

    print('Mesh Attributes:')
    print_records(attributes)
    print()

    print('Shader Tags:')
    print_records(tags, index=0)
