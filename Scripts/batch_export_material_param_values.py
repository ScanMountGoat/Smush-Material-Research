import os
import sys
import xml.etree.ElementTree as ET

def parse_xml(file_text, values_by_param):
    root = ET.fromstring(file_text)
    for material_node in root:
        for param_node in material_node:
            # Group values based on the material parameter name.
            param_name = param_node.attrib['name']
            if param_name not in values_by_param:
                values_by_param[param_name] = []

            # Convert nested value nodes to a comma delimited format for easier parsing later.
            # Some materials have missing texture names for some reason, so check for null.
            values = [node.text for node in param_node[0]]
            if not None in values:
                values_text = ','.join(values)
                values_by_param[param_name].append(values_text)


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print(f'Usage: {sys.argv[0]} <source folder> <destination folder>')
        exit(1)

    source_folder = os.path.abspath(sys.argv[1])
    destination_folder = os.path.abspath(sys.argv[2])

    values_by_param = {}

    # Parse all the xml files to find all possible values for each material parameter.
    for root, directories, file_paths in os.walk(source_folder):
        for path in file_paths:
            if '.xml' in path:
                abs_path = os.path.join(source_folder, path)
                with open(abs_path, 'r') as file:
                    data = file.read().encode('utf-16-be')
                    parse_xml(data, values_by_param)

    # Output the parameter values to corresponding files in the destination folder.
    for param_name in values_by_param:
        output_path = os.path.join(destination_folder,param_name + '_values.txt')
        with open(output_path,"w") as file:
            for value in values_by_param[param_name]:
                file.write(value + '\n')