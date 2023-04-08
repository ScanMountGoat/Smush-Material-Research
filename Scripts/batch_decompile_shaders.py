import os
import sys
import subprocess
from multiprocessing import Pool


def decompile_shader(params):
    root, f, destination_folder, shader_tools = params
    print(f)
    abs_path = os.path.join(root, f)

    output_filename = f.replace('.bin', '.glsl')
    output_path = os.path.join(destination_folder, output_filename)

    subprocess.run([shader_tools, abs_path, output_path])


if __name__ == '__main__':
    if len(sys.argv) != 4:
        print(
            f'Usage: python {sys.argv[0]} <Ryujinx.ShaderTools.exe> <Shader.bin Source Folder> <Destination Folder>')
        exit(1)

    shader_tools = os.path.abspath(sys.argv[1])
    source_folder = os.path.abspath(sys.argv[2])
    destination_folder = os.path.abspath(sys.argv[3])
    if not os.path.exists(destination_folder):
        os.makedirs(destination_folder)

    shader_files = []
    for root, directories, files in os.walk(source_folder):
        for f in files:
            if '.bin' in f:
                shader_files.append((root, f, destination_folder, shader_tools))

    # Use a pool of processes to speed this up.
    pool = Pool(8)
    pool.map(decompile_shader, shader_files)
