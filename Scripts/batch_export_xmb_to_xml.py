import os
import sys
import subprocess

if __name__ == '__main__':
    if len(sys.argv) != 4:
        print(f'Usage: python {sys.argv[0]} <XMBDec.py> <XMB source folder> <destination folder>')
        print(f'example: python {sys.argv[0]} XMBDec.py "Xmb Inputs" "XML Output"')
        exit(1)

    script = os.path.abspath(sys.argv[1])
    source_folder = os.path.abspath(sys.argv[2])
    destination_folder = os.path.abspath(sys.argv[3])
    if not os.path.exists(destination_folder):
        os.makedirs(destination_folder)

    for root, directories, files in os.walk(source_folder):
        for f in files:
            if '.xmb' in f:
                abs_path = os.path.join(root, f)

                # Ex: /a/b/c.xmb -> a_b_c.xml
                output_filename = abs_path.replace(source_folder, "").replace(os.path.sep, "_").replace('.xmb', '.xml')[1:]
                output_path = os.path.join(destination_folder, output_filename)
               
                # Run the python script to convert.
                subprocess.Popen([sys.executable, script, abs_path, '-o', output_path])
