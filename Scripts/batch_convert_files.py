import os
import sys
import subprocess

if __name__ == '__main__':
    if len(sys.argv) != 4:
        print(f'Usage: python {sys.argv[0]} <executable> <input extension> <output extension> <XMB source folder> <destination folder>')
        print(f'example: python {sys.argv[0]} xmb.exe .xmb .xml "Xmb Inputs" "XML Output"')
        exit(1)

    exe_path = os.path.abspath(sys.argv[1])
    extension = os.path.abspath(sys.argv[1])
    output_extension = os.path.abspath(sys.argv[2])
    source_folder = os.path.abspath(sys.argv[3])
    destination_folder = os.path.abspath(sys.argv[4])
    if not os.path.exists(destination_folder):
        os.makedirs(destination_folder)

    for root, directories, files in os.walk(source_folder):
        for f in files:
            if extension in f:
                abs_path = os.path.join(root, f)

                # Ex: /a/b/c.xmb -> a_b_c.xml
                output_filename = abs_path.replace(source_folder, "").replace(os.path.sep, "_").replace(extension, output_extension)[1:]
                output_path = os.path.join(destination_folder, output_filename)
               
                # Run the xmb executable to convert.
                subprocess.Popen([exe_path, abs_path, output_path])
