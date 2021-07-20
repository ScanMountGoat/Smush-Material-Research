import os
import sys
import xml.etree.ElementTree as ET
import argparse

def find_material(file_text, parameters_to_find):
    root = ET.fromstring(file_text)
    for material_node in root:
        # Find the name and label for materials including all the desired parameters.
        parameter_names = [node.attrib['name'] for node in material_node]
        if all(p in parameter_names for p in parameters_to_find):
            name = None
            if 'name' in material_node.attrib:
                name = material_node.attrib['name']
            elif 'shaderLabel' in material_node.attrib:
                name = material_node.attrib['shaderLabel']
            else:
                raise Exception("Material element has no 'name' or 'shaderLabel' attribute")
                
            label = None
            if 'label' in material_node.attrib:
                label = material_node.attrib['label']
            elif 'materialLabel' in material_node.attrib:
                label = material_node.attrib['materialLabel']
            else:
                raise Exception("Material element has no 'label' or 'materialLabel' attribute")
            
            yield (name, label)

    return None


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='print materials with specified parameters')
    parser.add_argument('--dir', type=str, help='the directory in which to look for xml files', required=True)
    parser.add_argument('--params', nargs="+", help="space separated list of the parameters that should be included. if not specified, all materials will be printed.", default=[])
    parser.add_argument('--first', action='store_true', help="only print the first material in the file", default=False, required=False)
    args = parser.parse_args()

    source_folder = os.path.abspath(args.dir)

    # Parse all the xml files in the provided folder.
    for root, directories, file_paths in os.walk(source_folder):
        for path in file_paths:
            if '.xml' in path:
                abs_path = os.path.join(root, path)
                with open(abs_path, 'r') as file:
                    data = file.read().encode('utf-16-be')
                    try:
                        materials = find_material(data, args.params)
                        if materials is not None:
                            for m in materials:
                                print(m, path)
                                if args.first:
                                    break
                        
                    except Exception as e:
                        print(f"Failed to parse XML file {abs_path}: {e}")
