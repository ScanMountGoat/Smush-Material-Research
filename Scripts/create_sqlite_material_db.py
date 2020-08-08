import os
import sys
import xml.etree.ElementTree as ET
import csv
import sqlite3
import time

param_names = ['BlendState0', 'CustomBoolean0', 'CustomBoolean1', 'CustomBoolean10', 'CustomBoolean11', 'CustomBoolean12', 
'CustomBoolean2', 'CustomBoolean3', 'CustomBoolean4', 'CustomBoolean5', 'CustomBoolean6', 'CustomBoolean7', 'CustomBoolean8', 
'CustomBoolean9', 'CustomFloat0', 'CustomFloat1', 'CustomFloat10', 'CustomFloat11', 'CustomFloat12', 'CustomFloat16', 'CustomFloat17',
 'CustomFloat18', 'CustomFloat19', 'CustomFloat4', 'CustomFloat6', 'CustomFloat8', 'CustomVector0', 'CustomVector1', 'CustomVector10', 
 'CustomVector11', 'CustomVector13', 'CustomVector14', 'CustomVector15', 'CustomVector16', 'CustomVector18', 'CustomVector19', 'CustomVector2', 
 'CustomVector20', 'CustomVector21', 'CustomVector22', 'CustomVector23', 'CustomVector24', 'CustomVector27', 'CustomVector29', 'CustomVector3', 
 'CustomVector30', 'CustomVector31', 'CustomVector32', 'CustomVector33', 'CustomVector34', 'CustomVector35', 'CustomVector37', 'CustomVector38',
  'CustomVector39', 'CustomVector4', 'CustomVector40', 'CustomVector42', 'CustomVector43', 'CustomVector44', 'CustomVector45', 'CustomVector46', 
  'CustomVector47', 'CustomVector5', 'CustomVector6', 'CustomVector7', 'CustomVector8', 'CustomVector9', 'RasterizerState0', 'Sampler0', 'Sampler1', 
  'Sampler10', 'Sampler11', 'Sampler12', 'Sampler13', 'Sampler14', 'Sampler16', 'Sampler2', 'Sampler3', 'Sampler4', 'Sampler5', 'Sampler6', 
  'Sampler7', 'Sampler8', 'Sampler9', 'Texture0', 'Texture1', 'Texture10', 'Texture11', 'Texture12', 'Texture13', 'Texture14', 'Texture16', 'Texture2', 
  'Texture3', 'Texture4', 'Texture5', 'Texture6', 'Texture7', 'Texture8', 'Texture9', 'UvTransform0', 'UvTransform1', 'UvTransform14', 'UvTransform2', 
  'UvTransform3', 'UvTransform4', 'UvTransform5', 'UvTransform6', 'UvTransform7', 'UvTransform9']

create_matl_table = """CREATE TABLE "Matl" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT)
)"""

create_material_table = """CREATE TABLE "Material" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"MatlID"	INTEGER NOT NULL,
	"MaterialLabel"	TEXT NOT NULL,
	"ShaderLabel"	TEXT NOT NULL,
	PRIMARY KEY("ID"),
	FOREIGN KEY("MatlID") REFERENCES "Matl"("ID")
)"""

create_vector_table = """CREATE TABLE "CustomVectorParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	REAL NOT NULL,
	"Value2"	REAL NOT NULL,
	"Value3"	REAL NOT NULL,
	"Value4"	REAL NOT NULL,
	FOREIGN KEY("MaterialID") REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomBooleanParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"""

create_param_table = """CREATE TABLE "CustomParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT)
)"""

create_float_table = """CREATE TABLE "CustomFloatParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER,
	"MaterialID"	INTEGER NOT NULL,
	"Value"	INTEGER NOT NULL,
	FOREIGN KEY("MaterialID") REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"""

create_boolean_table = """CREATE TABLE "CustomBooleanParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	FOREIGN KEY("MaterialID") REFERENCES "Material"("ID")
)"""

create_texture_table = """CREATE TABLE "Texture" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value"	TEXT NOT NULL,
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	FOREIGN KEY("MaterialID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"""


def parse_xml(file_text, cursor, matl_id):
    custom_boolean_records = []
    custom_float_records = []
    custom_vector_records = []
    texture_records = []

    # Create records by parsing the XML.
    root = ET.fromstring(file_text)
    for material_node in root:
        shader_label = material_node.attrib['name']
        material_label = material_node.attrib['label']

        cursor.execute('INSERT INTO Material(MatlID, MaterialLabel, ShaderLabel) VALUES(?,?,?)', (matl_id, material_label, shader_label))
        material_id = cursor.lastrowid

        for param_node in material_node:
            param_name = param_node.attrib['name']
            param_id = param_names.index(param_name)
            values = [node.text for node in param_node[0]]

            if 'Vector' in param_name:
                custom_vector_records.append((param_id, material_id, float(values[0]), float(values[1]), float(values[2]), float(values[3])))
            elif 'Boolean' in param_name:
                custom_boolean_records.append((param_id, material_id, bool(param_node[0].text)))
            elif 'Float' in param_name:
                custom_float_records.append((param_id, material_id, float(param_node[0].text)))
            elif 'Texture' in param_name:
                texture_records.append((param_id, material_id, param_node[0].text))

    # Insert all records into database.
    cursor.executemany('INSERT INTO CustomBooleanParam(ParamID, MaterialID, Value) VALUES(?,?,?)', custom_boolean_records)
    cursor.executemany('INSERT INTO CustomFloatParam(ParamID, MaterialID, Value) VALUES(?,?,?)', custom_float_records)
    cursor.executemany('INSERT INTO CustomVectorParam(ParamID, MaterialID, Value1, Value2, Value3, Value4) VALUES(?,?,?,?,?,?)', custom_vector_records)
    cursor.executemany('INSERT INTO Texture(ParamID, MaterialID, Value) VALUES(?,?,?)', texture_records)


def create_index(cursor, table_name, column_name):
    index_sql = f'CREATE INDEX {table_name}_{column_name}_Index ON {table_name}({column_name})'
    cursor.execute(index_sql)


def process_xml_file(cursor, root, file):
    abs_path = os.path.join(root, file)
    with open(abs_path, 'r') as xml_file:
        cursor.execute('INSERT INTO Matl(FileName) VALUES(?)', (file,))
        matl_id = cursor.lastrowid

        # Workaround for opening XML files.
        file_text = xml_file.read().encode('utf-16-be')
        parse_xml(file_text, cursor, matl_id)


def create_material_database(source_folder, db_file):
    with sqlite3.connect(db_file) as con:
        cursor = con.cursor()
      
        cursor.execute(create_matl_table)
        cursor.execute(create_material_table)
        cursor.execute(create_vector_table)
        cursor.execute(create_param_table)
        cursor.execute(create_float_table)
        cursor.execute(create_boolean_table)
        cursor.execute(create_texture_table)

        # TODO: rasterizer state, blend state, samplers

        # The param names are in order starting with 0.
        cursor.executemany('INSERT INTO CustomParam(ID,Name) VALUES(?,?)', [(i,name) for i,name in enumerate(param_names)])


        for root, directories, files in os.walk(source_folder):
            for file in files:
                if '.xml' not in file:
                    continue
                
                process_xml_file(cursor, root, file)



        # Add indexes to improve query performance.
        create_index(cursor, 'Matl', 'FileName')
        create_index(cursor, 'Material', 'MatlID')
        create_index(cursor, 'Material', 'ShaderLabel')
        create_index(cursor, 'Texture', 'MaterialID')
        create_index(cursor, 'Texture', 'ParamID')
        create_index(cursor, 'CustomBooleanParam', 'MaterialID')
        create_index(cursor, 'CustomBooleanParam', 'ParamID')
        create_index(cursor, 'CustomFloatParam', 'MaterialID')
        create_index(cursor, 'CustomFloatParam', 'ParamID')
        create_index(cursor, 'CustomVectorParam', 'MaterialID')
        create_index(cursor, 'CustomVectorParam', 'ParamID')


if __name__ == '__main__':
    # TODO: Handle the case where the file already exists.
    # TODO: Print progress?
    if len(sys.argv) != 3:
        print(f'Usage: {sys.argv[0]} <MATL XML source folder> <SQLite DB>')
        exit(1)

    start_time = time.time()

    matl_xml_source_folder = sys.argv[1]
    db_file = sys.argv[2]

    create_material_database(matl_xml_source_folder, db_file)

    print(f'{db_file} created in {time.time() - start_time} seconds')
