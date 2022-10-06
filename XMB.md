# model.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| aurora_vision | number | Related to jumbotrons or other screen projections. | 0, 1 |
| bounding_scale | number | | 0.0 to 1000.0 |
| check_material | number | *Unused* | 0, 1 |
| compress_type | number | | 0, 1, 2, 4, 5 |
| draw | action | *Unused* | 0 |
| draw | buffer | *Unused* | 1 |
| draw | type | *Unused* | `main`, `normalmap` |
| draw_range | number | | 0 to 22 |
| drawing_order | number | | 0 to 255 |
| force_opaque | number | | 0, 1 |
| instancing | number | | 0, 1 |
| is_multi_sh | number | | 0, 1 |
| lightset | number | Determines which light set index to use from the stage's lighting file. Light set type is determined by the `model` type. | 0 to 15 |
| mirror | number | | 0x0000 |
| model | type | Determines what model type the `model.xmb` file is for. | `fighter`, `stage`, `effect`, `effect_main` (unused), `effect_near` (unused) |
| object | type | *Unused* | 2 |
| poison | number |  Determines if a stage model can appear under foggy or poisonous cloud conditions in spirit battles. | 0 = Invisible, 1 = Visible |
| pre_depth | number | | 1 |
| reflection | search | *Unused* | 100.0 |
| shadow | caster | Determines what type of shadow casting the model will use. | 0 = Disabled, 1 = Enabled, 2 = Enabled + Texture 0 Alpha Test, 3 = Shadow Only, 4 = Shadow Only + Texture 0 Alpha Test. Shadow programs listed in `/render/shader/nusampleeffectlibrary.nufxlb`. |
| shadow_bounding_box_offset | offset | | `(float, float, float)` with values from 9.0 to 100.0 |
| silhouette_type | number | | 0, 1 |
| stage_expansion_type | number | | 0, 1 |
| stage_ink_type | number | Determines how Splat Roller ink will be projected onto surfaces with ink meshes. | 0 = Flat, 1 = ???, 2 = Overhanging, 3 = ??? (used in `s13_e` in poke_unova) |
| stage_sh_priority | number | | 0, 1, 2, 3, 4, 5 |
| stage_transition_type | number | Determines if a stage model temporarily blooms or not after the Stage Morph transition. | 0 = Enables bloom, 1 = ??? (identical to 0?), 2 = Disables bloom |
| stage_transition_visible | number | Determines if a stage model is visible during its transition out in Stage Morph. | 0 = Invisible, 1 = Visible |
| stage_visible_type | number | Determines a stage model's visibility during effect background appearences. | 0 = Always visible, 1 = Disappears during effect backgrounds, 2 = Only visible during effect backgrounds |
| stencil_type | number | Stenciling states. | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 |
| this_light | action | | 0, False |
| this_light | color | | `float, float, float` with values from 0.0 to 1.0 |
| this_light | local_offset | | `float, float, float` with values from -100.0 to 0.0 |
| this_light | offset | | 0.0 |
| this_light | radius | | 9.0 to 100.0 |
| version | number | *Unused.* Identifies the XMB file format version. | 2 |

# lod.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| Mesh | parent | | Mesh names, ex: armhair |
| Mesh | value | | Mesh names, ex: armhair_high |
| Motion | value | | 0 |
| Ratio | value | | Values from 0.0 to 1.0 |
| text_node | _text | | No value? |
| Visibility | value | | true, false |

# effect_locator.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| entry | effect_name | | Names starting with STG_ |
| entry | expansion_type | | |
| entry | id | | 0 |
| entry | joint | | Bone names |
| entry | model | | Model names, ex: `dc_reaper_set` |
| entry | pos | | `float, float, float` with values from -1000.0 to 1000.0 |
| entry | rot | | `float, float, float` with angle values (probably radians) |
| entry | scale | | `float, float, float` with values from 0.0 to 200.0 |
| entry | type | | 0, 1 |
| entry | user_data | Some sort of bit flags? | 0, 1, 2, 3, 4, 5, 65536, 196608 |

# area_light.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| entry | col_ceiling | | `float, float, float` with values from 0.0 to 3.0 |
| entry | col_ground | | `float, float, float` with values from 0.0 to 1.0|
| entry | id | | Names, ex: groundGreen_01, red_17 |
| entry | pos | | `float, float, float` with values from -400.0 to 400.0 |
| entry | rot | | `float, float, float` with angle values (probably radians) |
| entry | scale | | `float, float, float` with values from 0.0 to 200.0 |
