import sys
import struct
import re

# regex
temp_pattern = re.compile(r'tmp\d{1,3}')
utof_pattern = re.compile(r'utof\(0x........U\)')


# Use floating point literals where possible.
def replace_hex_literals(line):
    result = line

    # utof(0U) -> 0.0
    if 'utof(0U)' in line:
        result = result.replace('utof(0U)', '0.0')

    # utof(0x3F800000U) -> 1.0
    for match in utof_pattern.findall(line):
        hex_string = match[match.index('x') + 1 : match.index('U')]
        new_float = struct.unpack('!f', bytes.fromhex(hex_string))[0]
        result = result.replace(match, str(new_float))
    return result


def update_temp_env(line, value_by_temp_var):
    if '=' in line:
        parts = line.split('=')
        temp_vars = temp_pattern.findall(parts[0])
        if len(temp_vars) != 0:
            # temp vars can be initialized with other temp vars.
            assignment_text = parts[1].strip().replace(';', '')
            assignment_text = replace_temp_vars(assignment_text, temp_var_env)

            value_by_temp_var[temp_vars[0]] = assignment_text
            return True

    return False

def replace_temp_vars(line, temp_var_env):
    # Replace any temp vars with their value from the environment.
    # This assumes temp names are unique.
    result = line
    for temp in temp_pattern.findall(line):
        if temp in temp_var_env:
            result = result.replace(temp, temp_var_env[temp])

    return result


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f'Usage: python {sys.argv[0]} <yuzu glsl shader> > <output file>')
        print(f'Ex: python {sys.argv[0]} shader.glsl > shader_clean.txt')
        exit(1)

    with open(sys.argv[1], 'r') as file:
        lines = file.readlines()
        indices_to_remove = []
        temp_var_env = {}

        for i in range(len(lines)):
            # Get rid of comments.
            if '//' in lines[i]:
                indices_to_remove.append(i)
                continue

            # Remove redundant boolean expressions.
            lines[i] = lines[i].replace(' && true', '')
            lines[i] = lines[i].replace('!(true)', 'false')

            lines[i] = replace_hex_literals(lines[i])

            # Store the temp variables value and delete the line.
            if update_temp_env(lines[i], temp_var_env):
                indices_to_remove.append(i)
                continue

            lines[i] = replace_temp_vars(lines[i], temp_var_env)

        # Don't include any deleted lines
        new_lines = [lines[i] for i in range(len(lines)) if i not in indices_to_remove]
        print(''.join(new_lines))                       
