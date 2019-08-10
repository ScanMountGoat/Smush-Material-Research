import bpy
import os
import xml.etree.ElementTree as ET

def get_rgba_value(param_node):
    r = float(param_node[0][0].text)
    g = float(param_node[0][1].text)
    b = float(param_node[0][2].text)
    a = float(param_node[0][3].text)
    return (r, g, b, a)

def get_float_value(param_node):
    return float(param_node[0].text)

def add_texture_node(mat, param_node, index, group_node, input_name, image_root_directory):
    tex_node = mat.node_tree.nodes.new('ShaderNodeTexImage')
    tex_node.location = (-700,260 + (-270 * index))

    image_path = os.path.join(image_root_directory, param_node[0][0].text + '.png')
    tex_node.image = bpy.data.images.load(filepath=image_path)

    mat.node_tree.links.new(tex_node.outputs['Color'], group_node.inputs[input_name])
    mat.node_tree.links.new(tex_node.outputs['Alpha'], group_node.inputs[input_name + ' Alpha'])

def create_nodes(mat, material_node): 
    # Clear existing nodes.
    mat.use_nodes = True
    nodes = mat.node_tree.nodes
    for every_node in nodes: 
        nodes.remove(every_node)

    # Use a node group to simplify the inputs.
    ultimate_shader_node = mat.node_tree.nodes.new("ShaderNodeGroup")
    ultimate_shader_node.node_tree = bpy.data.node_groups['SmashUltimateShader']
    ultimate_shader_node.width = 200
    ultimate_shader_node.location = (-250,0)

    # TODO: Don't hard code the image directory.
    image_root_directory = 'C:\\Users\\Jonathan\\Downloads\\Lucina Blender Texture Test'
    
    # Initialize the node group inputs
    for param_node in material_node:
        param_name = param_node.attrib['name']

        if param_name == 'Texture0':
            add_texture_node(mat, param_node, 0, ultimate_shader_node, 'Col1', image_root_directory)
        if param_name == 'Texture1':
            add_texture_node(mat, param_node, 1, ultimate_shader_node, 'Col2', image_root_directory)
        elif param_name == 'Texture6':
            add_texture_node(mat, param_node, 2, ultimate_shader_node, 'Prm', image_root_directory)
        elif param_name == 'Texture4':
            add_texture_node(mat, param_node, 3, ultimate_shader_node, 'Nor', image_root_directory)
        elif param_name == 'CustomFloat8':
            ultimate_shader_node.inputs['CustomFloat8'].default_value = 1.0 + get_float_value(param_node)
        elif param_name == 'CustomFloat10':
            ultimate_shader_node.inputs['CustomFloat10'].default_value = get_float_value(param_node)
        elif param_name == 'CustomVector11':
            ultimate_shader_node.inputs['CustomVector11'].default_value = get_rgba_value(param_node)

    # Create and link output.
    output_node = nodes.new('ShaderNodeOutputMaterial')
    output_node.location = (0,25)
    mat.node_tree.links.new(ultimate_shader_node.outputs['BSDF'], output_node.inputs['Surface'])

def parse_xml(file_text):
    root = ET.fromstring(file_text)
    for material_node in root:
        # Create a PBR material based on the xml.
        material_name = material_node.attrib['label']

        # Try to override existing materials if possible.
        blender_mat = bpy.data.materials.get(material_name)
        if blender_mat is None:
            blender_mat = bpy.data.materials.new(name=material_name)
        else:
            for node in blender_mat.node_tree.nodes:
                blender_mat.node_tree.nodes.remove(node) 

        create_nodes(blender_mat, material_node)



def create_materials_from_xml(context, filepath, use_some_setting):
    # Append the group from the .blend file if not found.
    if bpy.data.node_groups.get('SmashUltimateShader') is None:
        # TODO: Don't hard code these paths.
        # For the addon, these can be relative paths.
        print('Appending node groups...')
        section = '\\NodeTree\\'
        blend_file = 'C:\\Users\\Jonathan\\Downloads\\Lucina Blender Texture Test\\node group.blend'
        bpy.ops.wm.append(filepath=blend_file + section + 'SmashUltimateMaterial', filename='SmashUltimateShader', directory=blend_file + section)

    print('Creating materials from %s...' % (filepath))

    with open(filepath, 'r') as file:
        # Workaround for files not being parsed.
        data = file.read().encode('utf-16-be')
        parse_xml(data)

    print('Finished')
    return {'FINISHED'}


# ImportHelper is a helper class, defines filename and
# invoke() function which calls the file selector.
from bpy_extras.io_utils import ImportHelper
from bpy.props import StringProperty, BoolProperty, EnumProperty
from bpy.types import Operator


class ImportSomeData(Operator, ImportHelper):
    '''This appears in the tooltip of the operator and in the generated docs'''
    bl_idname = 'import_test.some_data'  # important since its how bpy.ops.import_test.some_data is constructed
    bl_label = 'Import Some Data'

    # ImportHelper mixin class uses this
    filename_ext = '.xml'

    filter_glob: StringProperty(
        default='*.xml',
        options={'HIDDEN'},
        maxlen=255,  # Max internal buffer length, longer would be clamped.
    )

    # List of operator properties, the attributes will be assigned
    # to the class instance from the operator settings before calling.
    use_setting: BoolProperty(
        name='Example Boolean',
        description='Example Tooltip',
        default=True,
    )

    type: EnumProperty(
        name='Example Enum',
        description='Choose between two items',
        items=(
            ('OPT_A', 'First Option', 'Description one'),
            ('OPT_B', 'Second Option', 'Description two'),
        ),
        default='OPT_A',
    )

    def execute(self, context):
        return create_materials_from_xml(context, self.filepath, self.use_setting)


# Only needed if you want to add into a dynamic menu
def menu_func_import(self, context):
    self.layout.operator(ImportSomeData.bl_idname, text='Text Import Operator')


def register():
    bpy.utils.register_class(ImportSomeData)

def unregister():
    bpy.utils.unregister_class(ImportSomeData)


if __name__ == '__main__':
    register()

    # test call
    bpy.ops.import_test.some_data('INVOKE_DEFAULT')
