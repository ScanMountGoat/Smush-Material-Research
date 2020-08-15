import sys
import os
import sqlite3

select_vector_params_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, CustomParam.Name as ParamName
FROM CustomVectorParam
INNER JOIN Material
	ON CustomVectorParam.MaterialID = Material.ID
INNER JOIN CustomParam
	ON CustomVectorParam.ParamID = CustomParam.ID
ORDER BY Shader"""

select_bool_params_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, CustomParam.Name as ParamName
FROM CustomBooleanParam
INNER JOIN Material
	ON CustomBooleanParam.MaterialID = Material.ID
INNER JOIN CustomParam
	ON CustomBooleanParam.ParamID = CustomParam.ID
ORDER BY Shader"""

select_float_params_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, CustomParam.Name as ParamName
FROM CustomFloatParam
INNER JOIN Material
	ON CustomFloatParam.MaterialID = Material.ID
INNER JOIN CustomParam
	ON CustomFloatParam.ParamID = CustomParam.ID
ORDER BY Shader"""

select_textures_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, CustomParam.Name as ParamName
FROM Texture
INNER JOIN Material
	ON Texture.MaterialID = Material.ID
INNER JOIN CustomParam
	ON Texture.ParamID = CustomParam.ID
ORDER BY Shader"""

select_attributes_sql = """SELECT DISTINCT SUBSTR(ShaderLabel, 1, 24) as Shader, MeshAttribute
FROM ShaderMeshAttribute
ORDER BY Shader"""

select_tags_sql = """SELECT DISTINCT SUBSTR(Material.ShaderLabel, 1, 24) as Shader, SUBSTR(ShaderLabel, 25) as Tag
FROM Material
ORDER BY Shader"""

select_shaders_sql = """SELECT DISTINCT SUBSTR(ShaderLabel, 1,24) as Shader FROM Material"""

def get_values(con):
    bool_params = con.execute(select_bool_params_sql).fetchall()
    float_params = con.execute(select_float_params_sql).fetchall()
    vector_params = con.execute(select_vector_params_sql).fetchall()
    attributes = con.execute(select_attributes_sql).fetchall()
    texture_params = con.execute(select_textures_sql).fetchall()
    tags = con.execute(select_tags_sql).fetchall()

    return bool_params, float_params, vector_params, texture_params, attributes, tags


def write_records(file, records, index=1):
    for record in records:
        file.write(f'{record[index]}\n')


def get_shaders(con):
    records = con.execute(select_shaders_sql).fetchall()
    return [record[0] for record in records]


if __name__ == '__main__':  
    if len(sys.argv) != 3:
        print('Usage:')
        print(f'\tpython {sys.argv[0]} <Smush Materials SQLite DB> <Destination Folder>')
        exit(1)

    db_file = sys.argv[1]
    destination_folder = sys.argv[2]

    # It's faster to read the data all at once from SQLite and then sort in python.
    with sqlite3.connect(db_file) as con:
        bool_params, float_params, vector_params, texture_params, attributes, tags = get_values(con)
        for shader in get_shaders(con):
            output_path = os.path.join(destination_folder,f'{shader}_info.txt')
            with open(output_path,"w") as file:
                file.write('Material Parameters:\n')
                write_records(file, [record for record in bool_params if record[0] == shader])
                write_records(file, [record for record in float_params if record[0] == shader])
                write_records(file, [record for record in vector_params if record[0] == shader])
                file.write('\n')

                file.write('Textures:\n')
                write_records(file, [record for record in texture_params if record[0] == shader])
                file.write('\n')

                file.write('Mesh Attributes:\n')
                write_records(file, [record for record in attributes if record[0] == shader])
                file.write('\n')

                file.write('Shader Tags:\n')
                write_records(file, [record for record in tags if record[0] == shader])
