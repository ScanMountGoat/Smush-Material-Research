import re
hex_pattern = re.compile(r'0x[\dA-F]{1,3}')

lines = []
with open('Material Parameters.md', 'r') as file:
    lines = file.readlines()
    for i in range(len(lines)):
        for hex_string in hex_pattern.findall(lines[i]):
            new_value = str(int(hex_string, 16))
            lines[i] = lines[i].replace(hex_string, f'{new_value} ({hex_string})')

for line in lines:
    print(line[:-1])
