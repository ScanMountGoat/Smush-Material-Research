import os
import sys
import subprocess

if __name__ == '__main__':
    if len(sys.argv) < 6:
        print(f'Usage: {sys.argv[0]} <converter> <source folder> <destination folder> <input extension> <output extension>')
        exit(1)

    xml_exe = os.path.abspath(sys.argv[1])
    source_folder = os.path.abspath(sys.argv[2])
    destination_folder = os.path.abspath(sys.argv[3])
    if not os.path.exists(destination_folder):
        os.makedirs(destination_folder)

    input_extension = sys.argv[4]
    output_extension = sys.argv[5]

    for root, directories, files in os.walk(source_folder):
        for f in files:
            if input_extension in f:
                abs_path = os.path.join(root, f)

                # Ex: /a/b/c.numatb -> a_b_c.xml
                output_filename = abs_path.replace(source_folder, "").replace(os.path.sep, "_").replace(input_extension,output_extension)[1:]
                output_path = os.path.join(destination_folder, output_filename)
               
                # Execute the converter exe using the given paths.
                subprocess.Popen([xml_exe, abs_path, output_path])
