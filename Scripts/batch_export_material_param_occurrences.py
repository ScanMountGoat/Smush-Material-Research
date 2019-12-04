import os
import sys
import xml.etree.ElementTree as ET

param_names = ['BlendState0', 'CustomBoolean0', 'CustomBoolean1', 'CustomBoolean10', 'CustomBoolean11', 'CustomBoolean12', 'CustomBoolean2', 'CustomBoolean3', 'CustomBoolean4', 'CustomBoolean5', 'CustomBoolean6', 'CustomBoolean7', 'CustomBoolean8', 'CustomBoolean9', 'CustomFloat0', 'CustomFloat1', 'CustomFloat10', 'CustomFloat11', 'CustomFloat12', 'CustomFloat16', 'CustomFloat17', 'CustomFloat18', 'CustomFloat19', 'CustomFloat4', 'CustomFloat6', 'CustomFloat8', 'CustomVector0', 'CustomVector1', 'CustomVector10', 'CustomVector11', 'CustomVector13', 'CustomVector14', 'CustomVector15', 'CustomVector16', 'CustomVector18', 'CustomVector19', 'CustomVector2', 'CustomVector20', 'CustomVector21', 'CustomVector22', 'CustomVector23', 'CustomVector24', 'CustomVector27', 'CustomVector29', 'CustomVector3', 'CustomVector30', 'CustomVector31', 'CustomVector32', 'CustomVector33', 'CustomVector34', 'CustomVector35', 'CustomVector37', 'CustomVector38', 'CustomVector39', 'CustomVector4', 'CustomVector40', 'CustomVector42', 'CustomVector43', 'CustomVector44', 'CustomVector45', 'CustomVector46', 'CustomVector47', 'CustomVector5', 'CustomVector6', 'CustomVector7', 'CustomVector8', 'CustomVector9', 'RasterizerState0', 'Sampler0', 'Sampler1', 'Sampler10', 'Sampler11', 'Sampler12', 'Sampler13', 'Sampler14', 'Sampler16', 'Sampler2', 'Sampler3', 'Sampler4', 'Sampler5', 'Sampler6', 'Sampler7', 'Sampler8', 'Sampler9', 'Texture0', 'Texture1', 'Texture10', 'Texture11', 'Texture12', 'Texture13', 'Texture14', 'Texture16', 'Texture2', 'Texture3', 'Texture4', 'Texture5', 'Texture6', 'Texture7', 'Texture8', 'Texture9', 'UvTransform0', 'UvTransform1', 'UvTransform14', 'UvTransform2', 'UvTransform3', 'UvTransform4', 'UvTransform5', 'UvTransform6', 'UvTransform7', 'UvTransform9']

def parse_xml(file_text, rows):
    root = ET.fromstring(file_text)
    for material_node in root:
        # 1 if parameter is present and 0 otherwise.
        row_values = ['0'] * len(param_names)
        for param_node in material_node:
            name = param_node.attrib['name']
            row_values[param_names.index(name)] = '1'
        rows.append(','.join(row_values))


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print(f'Usage: {sys.argv[0]} <source folder> <destination file>')
        exit(1)

    source_folder = os.path.abspath(sys.argv[1])
    destination_file = os.path.abspath(sys.argv[2])

    # Add csv header.
    rows = [','.join(param_names)]

    # Parse all the xml files to find all possible values for each material parameter.
    for root, directories, file_paths in os.walk(source_folder):
        for path in file_paths:
            if '.xml' in path:
                abs_path = os.path.join(source_folder, path)
                with open(abs_path, 'r') as file:
                    data = file.read().encode('utf-16-be')
                    parse_xml(data, rows)

    # Write results to a file.
    with open(destination_file, 'w') as file:
        file.write('\n'.join(rows))