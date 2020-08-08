import os
import sys
import xml.etree.ElementTree as ET
import csv
import sqlite3
import time

param_id_by_param_name = {
    "Diffuse": 0,
    "Specular": 1,
    "Ambient": 2,
    "BlendMap": 3,
    "Transparency": 4,
    "DiffuseMapLayer1": 5,
    "CosinePower": 6,
    "SpecularPower": 7,
    "Fresnel": 8,
    "Roughness": 9,
    "EmissiveScale": 10,
    "EnableDiffuse": 11,
    "EnableSpecular": 12,
    "EnableAmbient": 13,
    "DiffuseMapLayer2": 14,
    "EnableTransparency": 15,
    "EnableOpacity": 16,
    "EnableCosinePower": 17,
    "EnableSpecularPower": 18,
    "EnableFresnel": 19,
    "EnableRoughness": 20,
    "EnableEmissiveScale": 21,
    "WorldMatrix": 22,
    "ViewMatrix": 23,
    "ProjectionMatrix": 24,
    "WorldViewMatrix": 25,
    "ViewInverseMatrix": 26,
    "ViewProjectionMatrix": 27,
    "WorldViewProjectionMatrix": 28,
    "WorldInverseTransposeMatrix": 29,
    "DiffuseMap": 30,
    "SpecularMap": 31,
    "AmbientMap": 32,
    "EmissiveMap": 33,
    "SpecularMapLayer1": 34,
    "TransparencyMap": 35,
    "NormalMap": 36,
    "DiffuseCubeMap": 37,
    "ReflectionMap": 38,
    "ReflectionCubeMap": 39,
    "RefractionMap": 40,
    "AmbientOcclusionMap": 41,
    "LightMap": 42,
    "AnisotropicMap": 43,
    "RoughnessMap": 44,
    "ReflectionMask": 45,
    "OpacityMask": 46,
    "UseDiffuseMap": 47,
    "UseSpecularMap": 48,
    "UseAmbientMap": 49,
    "UseEmissiveMap": 50,
    "UseTranslucencyMap": 51,
    "UseTransparencyMap": 52,
    "UseNormalMap": 53,
    "UseDiffuseCubeMap": 54,
    "UseReflectionMap": 55,
    "UseReflectionCubeMap": 56,
    "UseRefractionMap": 57,
    "UseAmbientOcclusionMap": 58,
    "UseLightMap": 59,
    "UseAnisotropicMap": 60,
    "UseRoughnessMap": 61,
    "UseReflectionMask": 62,
    "UseOpacityMask": 63,
    "DiffuseSampler": 64,
    "SpecularSampler": 65,
    "NormalSampler": 66,
    "ReflectionSampler": 67,
    "SpecularMapLayer2": 68,
    "NormalMapLayer1": 69,
    "NormalMapBc5": 70,
    "NormalMapLayer2": 71,
    "RoughnessMapLayer1": 72,
    "RoughnessMapLayer2": 73,
    "UseDiffuseUvTransform1": 74,
    "UseDiffuseUvTransform2": 75,
    "UseSpecularUvTransform1": 76,
    "UseSpecularUvTransform2": 77,
    "UseNormalUvTransform1": 78,
    "UseNormalUvTransform2": 79,
    "ShadowDepthBias": 80,
    "ShadowMap0": 81,
    "ShadowMap1": 82,
    "ShadowMap2": 83,
    "ShadowMap3": 84,
    "ShadowMap4": 85,
    "ShadowMap5": 86,
    "ShadowMap6": 87,
    "ShadowMap7": 88,
    "CastShadow": 89,
    "ReceiveShadow": 90,
    "ShadowMapSampler": 91,
    "Texture0": 92,
    "Texture1": 93,
    "Texture2": 94,
    "Texture3": 95,
    "Texture4": 96,
    "Texture5": 97,
    "Texture6": 98,
    "Texture7": 99,
    "Texture8": 100,
    "Texture9": 101,
    "Texture10": 102,
    "Texture11": 103,
    "Texture12": 104,
    "Texture13": 105,
    "Texture14": 106,
    "Texture15": 107,
    "Sampler0": 108,
    "Sampler1": 109,
    "Sampler2": 110,
    "Sampler3": 111,
    "Sampler4": 112,
    "Sampler5": 113,
    "Sampler6": 114,
    "Sampler7": 115,
    "Sampler8": 116,
    "Sampler9": 117,
    "Sampler10": 118,
    "Sampler11": 119,
    "Sampler12": 120,
    "Sampler13": 121,
    "Sampler14": 122,
    "Sampler15": 123,
    "CustomBuffer0": 124,
    "CustomBuffer1": 125,
    "CustomBuffer2": 126,
    "CustomBuffer3": 127,
    "CustomBuffer4": 128,
    "CustomBuffer5": 129,
    "CustomBuffer6": 130,
    "CustomBuffer7": 131,
    "CustomMatrix0": 132,
    "CustomMatrix1": 133,
    "CustomMatrix2": 134,
    "CustomMatrix3": 135,
    "CustomMatrix4": 136,
    "CustomMatrix5": 137,
    "CustomMatrix6": 138,
    "CustomMatrix7": 139,
    "CustomMatrix8": 140,
    "CustomMatrix9": 141,
    "CustomMatrix10": 142,
    "CustomMatrix11": 143,
    "CustomMatrix12": 144,
    "CustomMatrix13": 145,
    "CustomMatrix14": 146,
    "CustomMatrix15": 147,
    "CustomMatrix16": 148,
    "CustomMatrix17": 149,
    "CustomMatrix18": 150,
    "CustomMatrix19": 151,
    "CustomVector0": 152,
    "CustomVector1": 153,
    "CustomVector2": 154,
    "CustomVector3": 155,
    "CustomVector4": 156,
    "CustomVector5": 157,
    "CustomVector6": 158,
    "CustomVector7": 159,
    "CustomVector8": 160,
    "CustomVector9": 161,
    "CustomVector10": 162,
    "CustomVector11": 163,
    "CustomVector12": 164,
    "CustomVector13": 165,
    "CustomVector14": 166,
    "CustomVector15": 167,
    "CustomVector16": 168,
    "CustomVector17": 169,
    "CustomVector18": 170,
    "CustomVector19": 171,
    "CustomColor0": 172,
    "CustomColor1": 173,
    "CustomColor2": 174,
    "CustomColor3": 175,
    "CustomColor4": 176,
    "CustomColor5": 177,
    "CustomColor6": 178,
    "CustomColor7": 179,
    "CustomColor8": 180,
    "CustomColor9": 181,
    "CustomColor10": 182,
    "CustomColor11": 183,
    "CustomColor12": 184,
    "CustomColor13": 185,
    "CustomColor14": 186,
    "CustomColor15": 187,
    "CustomColor16": 188,
    "CustomColor17": 189,
    "CustomColor18": 190,
    "CustomColor19": 191,
    "CustomFloat0": 192,
    "CustomFloat1": 193,
    "CustomFloat2": 194,
    "CustomFloat3": 195,
    "CustomFloat4": 196,
    "CustomFloat5": 197,
    "CustomFloat6": 198,
    "CustomFloat7": 199,
    "CustomFloat8": 200,
    "CustomFloat9": 201,
    "CustomFloat10": 202,
    "CustomFloat11": 203,
    "CustomFloat12": 204,
    "CustomFloat13": 205,
    "CustomFloat14": 206,
    "CustomFloat15": 207,
    "CustomFloat16": 208,
    "CustomFloat17": 209,
    "CustomFloat18": 210,
    "CustomFloat19": 211,
    "CustomInteger0": 212,
    "CustomInteger1": 213,
    "CustomInteger2": 214,
    "CustomInteger3": 215,
    "CustomInteger4": 216,
    "CustomInteger5": 217,
    "CustomInteger6": 218,
    "CustomInteger7": 219,
    "CustomInteger8": 220,
    "CustomInteger9": 221,
    "CustomInteger10": 222,
    "CustomInteger11": 223,
    "CustomInteger12": 224,
    "CustomInteger13": 225,
    "CustomInteger14": 226,
    "CustomInteger15": 227,
    "CustomInteger16": 228,
    "CustomInteger17": 229,
    "CustomInteger18": 230,
    "CustomInteger19": 231,
    "CustomBoolean0": 232,
    "CustomBoolean1": 233,
    "CustomBoolean2": 234,
    "CustomBoolean3": 235,
    "CustomBoolean4": 236,
    "CustomBoolean5": 237,
    "CustomBoolean6": 238,
    "CustomBoolean7": 239,
    "CustomBoolean8": 240,
    "CustomBoolean9": 241,
    "CustomBoolean10": 242,
    "CustomBoolean11": 243,
    "CustomBoolean12": 244,
    "CustomBoolean13": 245,
    "CustomBoolean14": 246,
    "CustomBoolean15": 247,
    "CustomBoolean16": 248,
    "CustomBoolean17": 249,
    "CustomBoolean18": 250,
    "CustomBoolean19": 251,
    "UvTransform0": 252,
    "UvTransform1": 253,
    "UvTransform2": 254,
    "UvTransform3": 255,
    "UvTransform4": 256,
    "UvTransform5": 257,
    "UvTransform6": 258,
    "UvTransform7": 259,
    "UvTransform8": 260,
    "UvTransform9": 261,
    "UvTransform10": 262,
    "UvTransform11": 263,
    "UvTransform12": 264,
    "UvTransform13": 265,
    "UvTransform14": 266,
    "UvTransform15": 267,
    "DiffuseUvTransform1": 268,
    "DiffuseUvTransform2": 269,
    "SpecularUvTransform1": 270,
    "SpecularUvTransform2": 271,
    "NormalUvTransform1": 272,
    "NormalUvTransform2": 273,
    "DiffuseUvTransform": 274,
    "SpecularUvTransform": 275,
    "NormalUvTransform": 276,
    "UseDiffuseUvTransform": 277,
    "UseSpecularUvTransform": 278,
    "UseNormalUvTransform": 279,
    "BlendState0": 280,
    "BlendState1": 281,
    "BlendState2": 282,
    "BlendState3": 283,
    "BlendState4": 284,
    "BlendState5": 285,
    "BlendState6": 286,
    "BlendState7": 287,
    "BlendState8": 288,
    "BlendState9": 289,
    "BlendState10": 290,
    "RasterizerState0": 291,
    "RasterizerState1": 292,
    "RasterizerState2": 293,
    "RasterizerState3": 294,
    "RasterizerState4": 295,
    "RasterizerState5": 296,
    "RasterizerState6": 297,
    "RasterizerState7": 298,
    "RasterizerState8": 299,
    "RasterizerState9": 300,
    "RasterizerState10": 301,
    "ShadowColor": 302,
    "EmissiveMapLayer1": 303,
    "EmissiveMapLayer2": 304,
    "AlphaTestFunc": 305,
    "AlphaTestRef": 306,
    "Texture16": 307,
    "Texture17": 308,
    "Texture18": 309,
    "Texture19": 310,
    "Sampler16": 311,
    "Sampler17": 312,
    "Sampler18": 313,
    "Sampler19": 314,
    "CustomVector20": 315,
    "CustomVector21": 316,
    "CustomVector22": 317,
    "CustomVector23": 318,
    "CustomVector24": 319,
    "CustomVector25": 320,
    "CustomVector26": 321,
    "CustomVector27": 322,
    "CustomVector28": 323,
    "CustomVector29": 324,
    "CustomVector30": 325,
    "CustomVector31": 326,
    "CustomVector32": 327,
    "CustomVector33": 328,
    "CustomVector34": 329,
    "CustomVector35": 330,
    "CustomVector36": 331,
    "CustomVector37": 332,
    "CustomVector38": 333,
    "CustomVector39": 334,
    "CustomVector40": 335,
    "CustomVector41": 336,
    "CustomVector42": 337,
    "CustomVector43": 338,
    "CustomVector44": 339,
    "CustomVector45": 340,
    "CustomVector46": 341,
    "CustomVector47": 342,
    "CustomVector48": 343,
    "CustomVector49": 344,
    "CustomVector50": 345,
    "CustomVector51": 346,
    "CustomVector52": 347,
    "CustomVector53": 348,
    "CustomVector54": 349,
    "CustomVector55": 350,
    "CustomVector56": 351,
    "CustomVector57": 352,
    "CustomVector58": 353,
    "CustomVector59": 354,
    "CustomVector60": 355,
    "CustomVector61": 356,
    "CustomVector62": 357,
    "CustomVector63": 358,
    "UseBaseColorMap": 359,
    "UseMetallicMap": 360,
    "BaseColorMap": 361,
    "BaseColorMapLayer1": 362,
    "MetallicMap": 363,
    "MetallicMapLayer1": 364,
    "DiffuseLightingAoOffset": 365,
}

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

create_blendstate_table = """CREATE TABLE "BlendState" (
	"ID"	INTEGER NOT NULL UNIQUE,
    "ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	INTEGER NOT NULL,
	"Value2"	INTEGER NOT NULL,
	"Value3"	INTEGER NOT NULL,
	"Value4"	INTEGER NOT NULL,
	"Value5"	INTEGER NOT NULL,
	"Value6"	INTEGER NOT NULL,
	"Value7"	INTEGER NOT NULL,
	"Value8"	INTEGER NOT NULL,
	"Value9"	INTEGER NOT NULL,
	"Value10"	INTEGER NOT NULL,
	"Value11"	INTEGER NOT NULL,
	"Value12"	INTEGER NOT NULL,
	FOREIGN KEY("MaterialID") REFERENCES "Material"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"""

create_rasterizerstate_table = """CREATE TABLE "RasterizerState" (
	"ID"	INTEGER NOT NULL UNIQUE,
    "ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	INTEGER NOT NULL,
	"Value2"	INTEGER NOT NULL,
	"Value3"	REAL NOT NULL,
	"Value4"	REAL NOT NULL,
	"Value5"	REAL NOT NULL,
	"Value6"	INTEGER NOT NULL,
	"Value7"	INTEGER NOT NULL,
	"Value8"	REAL NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("MaterialID") REFERENCES "Material"("ID")
)"""

create_sampler_table = """CREATE TABLE "Sampler" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	INTEGER NOT NULL,
	"Value2"	INTEGER NOT NULL,
	"Value3"	INTEGER NOT NULL,
	"Value4"	INTEGER NOT NULL,
	"Value5"	INTEGER NOT NULL,
	"Value6"	INTEGER NOT NULL,
	"Value7"	INTEGER NOT NULL,
	"Value8"	INTEGER NOT NULL,
	"Value9"	INTEGER NOT NULL,
	"Value10"	INTEGER NOT NULL,
	"Value11"	INTEGER NOT NULL,
	"Value12"	INTEGER NOT NULL,
	"Value13"	REAL NOT NULL,
	"Value14"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	FOREIGN KEY("MaterialID") REFERENCES "Material"("ID")
)"""

insert_custom_booleans = 'INSERT INTO CustomBooleanParam(ParamID, MaterialID, Value) VALUES(?,?,?)'
insert_custom_floats = 'INSERT INTO CustomFloatParam(ParamID, MaterialID, Value) VALUES(?,?,?)'
insert_custom_vectors = 'INSERT INTO CustomVectorParam(ParamID, MaterialID, Value1, Value2, Value3, Value4) VALUES(?,?,?,?,?,?)'
insert_textures = 'INSERT INTO Texture(ParamID, MaterialID, Value) VALUES(?,?,?)'
insert_samplers = 'INSERT INTO Sampler(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)'
insert_rasterizer_states = 'INSERT INTO RasterizerState(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8) VALUES(?,?,?,?,?,?,?,?,?,?)'
insert_blend_states = 'INSERT INTO BlendState(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?)'

def parse_xml(file_text, cursor, matl_id):
    custom_boolean_records = []
    custom_float_records = []
    custom_vector_records = []
    texture_records = []
    rasterizer_state_records = []
    sampler_records = []
    blend_state_records = []

    # Create records by parsing the XML.
    root = ET.fromstring(file_text)
    for material_node in root:
        shader_label = material_node.attrib['name']
        material_label = material_node.attrib['label']

        cursor.execute('INSERT INTO Material(MatlID, MaterialLabel, ShaderLabel) VALUES(?,?,?)', (matl_id, material_label, shader_label))
        material_id = cursor.lastrowid

        for param_node in material_node:
            param_name = param_node.attrib['name']
            param_id = param_id_by_param_name[param_name]
            values = [node.text for node in param_node[0]]

            if 'Vector' in param_name:
                custom_vector_records.append((param_id, material_id, values[0], values[1], values[2], values[3]))
            elif 'Boolean' in param_name:
                custom_boolean_records.append((param_id, material_id, param_node[0].text))
            elif 'Float' in param_name:
                custom_float_records.append((param_id, material_id, param_node[0].text))
            elif 'Texture' in param_name:
                texture_records.append((param_id, material_id, param_node[0].text))
            elif 'Sampler' in param_name:
                record = (param_id, material_id, values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8], values[9], values[10], values[11], values[12], values[13])
                sampler_records.append(record)
            elif 'BlendState' in param_name:
                record = (param_id, material_id, values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8], values[9], values[10], values[11])
                blend_state_records.append(record)
            elif 'RasterizerState' in param_name:
                record = (param_id, material_id, values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7])
                rasterizer_state_records.append(record)

    # Insert all records into database.
    cursor.executemany(insert_custom_booleans, custom_boolean_records)
    cursor.executemany(insert_custom_floats, custom_float_records)
    cursor.executemany(insert_custom_vectors, custom_vector_records)
    cursor.executemany(insert_textures, texture_records)
    cursor.executemany(insert_samplers, sampler_records)
    cursor.executemany(insert_rasterizer_states, rasterizer_state_records)
    cursor.executemany(insert_blend_states, blend_state_records)


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
      
        # Create database tables.
        cursor.execute(create_matl_table)
        cursor.execute(create_material_table)
        cursor.execute(create_param_table)
        cursor.execute(create_vector_table)
        cursor.execute(create_float_table)
        cursor.execute(create_boolean_table)
        cursor.execute(create_texture_table)
        cursor.execute(create_blendstate_table)
        cursor.execute(create_rasterizerstate_table)
        cursor.execute(create_sampler_table)

        # Use param ID as an enum with IDs starting at 0.
        cursor.executemany('INSERT INTO CustomParam(ID,Name) VALUES(?,?)', [(pair[1], pair[0]) for pair in param_id_by_param_name.items()])

        # Process all the xml files and fill the tables.
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
        create_index(cursor, 'Sampler', 'MaterialID')
        create_index(cursor, 'Sampler', 'ParamID')
        create_index(cursor, 'RasterizerState', 'MaterialID')
        create_index(cursor, 'BlendState', 'MaterialID')

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
