import os
import sys
import glob
import subprocess

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print('Usage:')
        print(f'\t{sys.argv[0]} <source folder> <destination folder>')
        exit(1)

    xml_exe = os.path.abspath(sys.argv[1])
    source_folder = os.path.abspath(sys.argv[2])
    destination_folder = os.path.abspath(sys.argv[3])

    for root, directories, files in os.walk(source_folder):
        for f in files:
            if '.xml' in f:
                print(f)