import os
import sys
import xml.etree.ElementTree as ET

def find_material(file_text, parameters_to_find):
    root = ET.fromstring(file_text)
    for material_node in root:
        # Find the name and label for materials including all the desired parameters.
        parameter_names = [node.attrib['name'] for node in material_node]
        if all(p in parameter_names for p in parameters_to_find):
            return (material_node.attrib['name'], material_node.attrib['label'])

    return None


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f'Usage: {sys.argv[0]} <xml material dump folder>')
        exit(1)

    source_folder = os.path.abspath(sys.argv[1])

    # Change this to whatever parameters should be included. 
    # Setting this to an empty list will print all materials.
    parameters_to_find = ['Texture4', 'Texture5', 'CustomVector3', 'CustomVector8']

    # Parse all the xml files in the provided folder.
    for root, directories, file_paths in os.walk(source_folder):
        for path in file_paths:
            if '.xml' in path:
                abs_path = os.path.join(root, path)
                with open(abs_path, 'r') as file:
                    data = file.read().encode('utf-16-be')
                    material = find_material(data, parameters_to_find)
                    if material is not None:
                        print(material, path)