import sys
import re

# regex
variable_pattern = re.compile(r'gpr[0-9]+|tmp[0-9]+|frag_color[0-9]+')

# Recursively finds the most recent lines where variables were assigned.
def find_assignments(lines, variable_to_find, assignment_by_line):
    for line_index in range(len(lines) - 1, -1, -1):
        # Skip costly recursion on lines that have already been processed.
        line_number = line_index + 1
        if line_number in assignment_by_line:
            continue

        # Only parse lines with assignments.
        line = lines[line_index][:-1].strip()
        if '=' not in line:
            continue

        # Find any variables on the line.
        variables = variable_pattern.findall(line)
        if len(variables) == 0:
            continue

        variable = variables[0]
        if variable != variable_to_find:
            continue

        # This line is an assignment, so add it the output.
        assignment_by_line[line_number] = line

        # Search any variables used in the assignment.
        assignment_variables = list(dict.fromkeys(variables[1:]))
        for assignment_variable in assignment_variables:
            find_assignments(lines[:line_index], assignment_variable, assignment_by_line)

        # Stop searching once the most recent assignment is found.
        return


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print(f'Usage: python {sys.argv[0]} <yuzu glsl shader> <line #> > <output file>')
        exit(1)

    line_number = int(sys.argv[2])

    with open(sys.argv[1], 'r') as file:
        lines = file.readlines()

        start_index = line_number - 1
        variables = variable_pattern.findall(lines[start_index])

        if len(variables) == 0 or '=' not in lines[start_index]:
            print(f'{lines[start_index]} does contain a variable assignment')
            exit(1)

        # Start with the current assignment.
        assignment_by_line = {}
        find_assignments(lines[:start_index+1], variables[0], assignment_by_line)

        # Print in ascending line order
        for line in sorted(assignment_by_line.keys()):
            print(f'{line}\t{assignment_by_line[line]}')
