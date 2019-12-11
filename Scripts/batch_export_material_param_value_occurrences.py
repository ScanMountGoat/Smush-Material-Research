import os
import sys

def parse_csv(path, count_by_value_by_param):
    # param_values.txt -> param
    param_name = os.path.basename(path).split('_')[0]

    if param_name not in count_by_value_by_param:
        count_by_value_by_param[param_name] = {}

    with open(path, 'r') as file:
        for line in file.readlines():
            line = line.replace('\n', '')
            if line not in count_by_value_by_param[param_name]:
                count_by_value_by_param[param_name][line] = 1
            else:
                count_by_value_by_param[param_name][line] += 1


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
            if '.txt' in path:
                abs_path = os.path.join(source_folder, path)
                parse_csv(abs_path, count_by_value_by_param)

    # Write value occurrences in descending order for each param to file.
    for param_name in count_by_value_by_param:
        destination_file = os.path.join(destination_folder, param_name + '_value_occurences.csv')
        with open(destination_file, 'w') as file:
            # Write CSV header
            if 'Vector' in param_name:
                file.write('X,Y,Z,W,Occurrences\n')
            elif 'UvTransform' in param_name:
                file.write('Unk1,Unk2,Unk3,Unk4,Unk5,Occurrences\n')
            elif 'BlendState' in param_name:
                file.write('Unk1,Unk2,Unk3,Unk4,Unk5,Unk6,Unk7,Unk8,Unk9,Unk10,Unk11,Unk12,Occurrences\n')
            elif 'RasterizerState' in param_name:
                file.write('Unk1,Unk2,Unk3,Unk4,Unk5,Unk6,Unk7,Unk8,Occurrences\n')
            elif 'Sampler' in param_name:
                file.write('Unk1,Unk2,Unk3,Unk4,Unk5,Unk6,Unk7,Unk8,Unk9,Unk10,Unk11,Unk12,Unk13,Unk14,Occurrences\n')
            else:
                file.write('Value,Occurrences\n')

            dict = count_by_value_by_param[param_name]
            for value in sorted(dict, key=dict.get, reverse=True):
                file.write(f'{value},{count_by_value_by_param[param_name][value]}\n')

