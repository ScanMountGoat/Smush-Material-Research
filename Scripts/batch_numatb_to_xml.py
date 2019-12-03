import os
import sys
import glob

if __name__ == '__main__':
    if len(sys.argv) < 4:
        print('Usage:')
        print(f'\t{sys.argv[0]} <MatLab.exe> <source folder> <destination folder>')
        exit(1)

    xml_exe = os.path.abspath(sys.argv[1])
    source_folder = os.path.abspath(sys.argv[2])
    destination_folder = os.path.abspath(sys.argv[3])

    # r=root, d=directories, f = files
    for root, directories, files in os.walk(source_folder):
        for f in files:
            if '.numatb' in f:
                abs_path = os.path.join(root, f)

                # Ex: /a/b/c.numatb -> a_b_c.xml
                output_path = abs_path.replace(source_folder, "").replace(os.path.sep, "_").replace(".numatb",".xml")[1:]

                print(f'{abs_path} {output_path}')
                # TODO: Execute the matlab exe using the given paths.
