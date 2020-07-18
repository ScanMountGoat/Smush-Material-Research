import sys
import struct
import re

# regex
temp_pattern = re.compile(r'tmp\d{1,3};')
utof_pattern = re.compile(r'utof\(0x........U\)')
ternary_pattern = re.compile(r'\(!\(true\)\s\?.*:\s')
ternary_pattern2 = re.compile(r'true\s\?.*:\s')
fma_pattern = re.compile(r'fma\(.*\)')

# Assign value using only one line.
def remove_temp(current_line, prev_line):
    assignment = prev_line[prev_line.index('=') + 1:]
    current_var = current_line[:current_line.index('=') + 1]
    return current_var + assignment

# Use floating point literals where possible.
def replace_hex_literals(line):
    result = line

    # utof(0x3F800000U) -> 1.0
    if 'utof(0U)' in line:
        result = result.replace('utof(0U)', '0.0')

    # utof(0x3F800000U) -> 1.0
    for match in utof_pattern.findall(line):
        hex_string = match[match.index('x') + 1 : match.index('U')]
        new_float = struct.unpack('!f', bytes.fromhex(hex_string))[0]
        result = result.replace(match, str(new_float))
    return result

# fma(a, b, c) -> a * b + c
def replace_fma(line):
    result = line
    for fma_section in fma_pattern.findall(line):
        parts = fma_section[4:-1].split(',')
        explicit_fma = f'{parts[0]} *{parts[1]} +{parts[2]}'
        result = result.replace(fma_section, explicit_fma)
    return result


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f'Usage: python {sys.argv[0]} <yuzu glsl shader> > <output file>')
        print(f'Ex: python {sys.argv[0]} shader.glsl > shader_clean.txt')
        exit(1)

    with open(sys.argv[1], 'r') as file:
        lines = file.readlines()
        indices_to_remove = []
        for i in range(len(lines)):
            lines[i] = replace_fma(lines[i])

            # Remove redundant boolean expressions.
            lines[i] = lines[i].replace(' && true', '')
            lines[i] = ternary_pattern.sub('', lines[i])
            lines[i] = ternary_pattern2.sub('', lines[i])

            # Get rid of comments.
            if '//' in lines[i]:
                indices_to_remove.append(i)
                continue

            lines[i] = replace_hex_literals(lines[i])

            # Combine the temp line and remove it
            if temp_pattern.search(lines[i]):
                lines[i] = remove_temp(lines[i], lines[i-1])
                indices_to_remove.append(i-1)

        # Don't include any deleted lines
        new_lines = [lines[i] for i in range(len(lines)) if i not in indices_to_remove]
        print(''.join(new_lines))                       
