import os
import sys
import json
import sqlite3
import time

create_shader_program_table = """CREATE TABLE "ShaderProgram" (
	"ID" INTEGER NOT NULL UNIQUE,
	"Name" TEXT NOT NULL UNIQUE,
	PRIMARY KEY("ID" AUTOINCREMENT)
)"""

create_vertex_attribute_table = """CREATE TABLE "VertexAttribute" (
	"ID" INTEGER NOT NULL UNIQUE,
    "ShaderProgramID" INTEGER NOT NULL,
	"Name" TEXT NOT NULL,
	"AttributeName" TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
    FOREIGN KEY("ShaderProgramID") REFERENCES "ShaderProgram"("ID")
)"""

create_material_parameter_table = """CREATE TABLE "MaterialParameter" (
	"ID" INTEGER NOT NULL UNIQUE,
    "ShaderProgramID" INTEGER NOT NULL,
	"ParamId" INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT)
    FOREIGN KEY("ShaderProgramID") REFERENCES "ShaderProgram"("ID")
)"""

insert_shader_program = 'INSERT INTO ShaderProgram(Name) VALUES(?)'
insert_vertex_attribute = 'INSERT INTO VertexAttribute(ShaderProgramID, Name, AttributeName) VALUES(?,?,?)'
insert_material_parameter = 'INSERT INTO MaterialParameter(ShaderProgramID, ParamId) VALUES(?,?)'


def create_index(cursor, table_name, column_name):
    index_sql = f'CREATE INDEX {table_name}_{column_name}_Index ON {table_name}({column_name})'
    cursor.execute(index_sql)


def create_database_tables(cursor):
    cursor.execute(create_shader_program_table)
    cursor.execute(create_vertex_attribute_table)
    cursor.execute(create_material_parameter_table)


def create_indices(cursor):
    # Add indexes to improve query performance.
    create_index(cursor, 'ShaderProgram', 'Name')
    create_index(cursor, 'VertexAttribute', 'ShaderProgramID')
    create_index(cursor, 'MaterialParameter', 'ShaderProgramID')


def fill_database(cursor, nuxf_file):
    with open(nuxf_file, 'r') as file:
        nufx = json.loads(file.read())
        for program in nufx['data']['Nufx']['programs']:
            program_name = program['name']
            
            # Assume each shader can use any of the render passes.
            # if '_opaque' in program_name or '_near' in program_name or '_far' in program_name or '_sort' in program_name:
            #     continue

            cursor.execute(insert_shader_program, (program_name,))
            program_id = cursor.lastrowid

            attribute_records = []
            for attribute in program['vertex_attributes']:
                attribute_records.append((program_id, attribute['name'], attribute['attribute_name']))
            cursor.executemany(insert_vertex_attribute, attribute_records)

            parameter_records = []
            for attribute in program['material_parameters']:
                parameter_records.append((program_id, attribute['param_id']))
            cursor.executemany(insert_material_parameter, parameter_records)


def create_shader_db(nuxf_file, db_file):
    with sqlite3.connect(db_file) as con:
        cursor = con.cursor()

        create_database_tables(cursor)
        fill_database(cursor, nuxf_file)
        create_indices(cursor)

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print(f'Usage: {sys.argv[0]} <nuflxb.json> <SQLite DB>')
        exit(1)

    start_time = time.time()

    nuxf_file = sys.argv[1]
    db_file = sys.argv[2]

    # Overwrite the existing database file.
    if os.path.exists(db_file):
        os.remove(db_file)

    create_shader_db(nuxf_file, db_file)

    # Shrink the size if possible.
    with sqlite3.connect(db_file) as con:
        con.execute('VACUUM')

    print(f'{db_file} created in {time.time() - start_time} seconds')
