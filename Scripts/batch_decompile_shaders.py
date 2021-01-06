import os
import sys
import subprocess

if __name__ == '__main__':
    if len(sys.argv) != 4:
        print(f'Usage: python {sys.argv[0]} <Ryujinx.ShaderTools.exe> <Shader.bin Source Folder> <Destination Folder>')
        exit(1)

    shader_tools = os.path.abspath(sys.argv[1])
    source_folder = os.path.abspath(sys.argv[2])
    destination_folder = os.path.abspath(sys.argv[3])
    if not os.path.exists(destination_folder):
        os.makedirs(destination_folder)

    for root, directories, files in os.walk(source_folder):
        for f in files:
            print(f)
            if '.bin' in f:
                abs_path = os.path.join(root, f)

                output_filename = f.replace('.bin','.glsl')
                output_path = os.path.join(destination_folder, output_filename)
               
                # Redirect the console output to a file.
                with open(output_path, 'w') as file:
                    subprocess.run([shader_tools, abs_path], stdout=file)
