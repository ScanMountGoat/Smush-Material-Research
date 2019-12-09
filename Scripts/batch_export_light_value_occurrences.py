import os
import sys
import json

def parse_json(path, count_by_value_by_param):
    with open(path, 'r') as file:
        nuanim = json.loads(file.read())
        if nuanim['Animations'] is None:
            return

        for animation in nuanim['Animations']:
            for node in animation['Nodes']:
                for track in node['Tracks']:
                    param_name = track['Name']
                    if param_name not in count_by_value_by_param:
                        count_by_value_by_param[param_name] = {}

                    for value in track['Values']:
                        # Convert dictionaries into a CSV-friendly format.
                        value_text = ''
                        if type(value) is dict:
                            value_text = ','.join(str(x) for x in value.values())
                        else:
                            value_text = str(value)

                        # Update the occurrence count for the current value.
                        if value_text not in count_by_value_by_param[param_name]:
                            count_by_value_by_param[param_name][value_text] = 1
                        else:
                            count_by_value_by_param[param_name][value_text] += 1


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print(f'Usage: {sys.argv[0]} <source folder> <destination folder>')
        exit(1)

    source_folder = os.path.abspath(sys.argv[1])
    destination_folder = os.path.abspath(sys.argv[2])
    if not os.path.exists(destination_folder):
        os.makedirs(destination_folder)

    count_by_value_by_param = {}

    # Parse all the xml files to find all possible values for each material parameter.
    for root, directories, file_paths in os.walk(source_folder):
        for path in file_paths:
            if '.json' in path:
                abs_path = os.path.join(source_folder, path)
                parse_json(abs_path, count_by_value_by_param)

    # Write value occurrences in descending order for each param to file.
    for param_name in count_by_value_by_param:
        destination_file = os.path.join(destination_folder, param_name + '_value_occurences.csv')
        with open(destination_file, 'w') as file:
            # Write CSV header
            if 'Transform' in param_name:
                file.write('X,Y,Z,Rx,Ry,Rz,Rw,Sx,Sy,Sz,CompensateScale,Occurrences\n')
            elif 'Vector' in param_name:
                file.write('X,Y,Z,W,Occurrences\n')
            else:
                file.write('Value,Occurrences\n')

            # Write values
            value_dict = count_by_value_by_param[param_name]
            for value in sorted(value_dict, key=value_dict.get, reverse=True):
                file.write(f'{value},{count_by_value_by_param[param_name][value]}\n')

