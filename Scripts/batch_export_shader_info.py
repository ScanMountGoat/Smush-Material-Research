import sys
import json
import os

if __name__ == '__main__':  
    if len(sys.argv) != 3:
        print('Usage:')
        print(f'\tpython {sys.argv[0]} <nuc2effectlibrary.nufxlb.json> <Destination Folder>')
        exit(1)

    nufx_file = sys.argv[1]
    destination_folder = sys.argv[2]

    # It's faster to read the data all at once from SQLite and then sort in python.
    with open(nufx_file, 'r') as file:
        nufxb = json.loads(file.read())

        for program in nufxb['data']['Nufx']['programs']['ProgramsV1']:
            program_name = program['name']
            output_path = os.path.join(destination_folder,f'{program_name}_info.txt')
            with open(output_path,"w") as file:
                file.write('=== Material Parameters ===\n')
                file.writelines([param['parameter_name'] + '\n' for param in program['material_parameters']])
                file.write('\n')

                file.write('=== Vertex Attributes ===\n')
                file.writelines([attr['attribute_name'] + '\n' for attr in program['vertex_attributes']])
                file.write('\n')
