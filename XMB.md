# model.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| aurora_vision | number | Related to jumbotrons or other screen projections.  | 0, 1 |
| bounding_scale | number | ??? | 0.0 to 1000.0 |
| check_material | number | | 0, 1 |
| compress_type | number | | 0, 1, 2, 4, 5 |
| draw | action | | 0 |
| draw | buffer | | 1 |
| draw | type | | main, normalmap |
| draw_range | number | | 0 to 22 |
| drawing_order | number | | 0 to 255 |
| force_opaque | number | | 0, 1 |
| instancing | number | | 0, 1 |
| is_multi_sh | number | | 0, 1 |
| lightset | number | Determines which light set to use from the lighting file. Light set type may be determined by the model type. | 0 to 10 |
| mirror | number | | 0x0000 |
| model | type | Determines what model type the model.xmb file is for. | effect, effect_main, effect_near, fighter, stage |
| object | type | | 2 |
| poison | number | | 0, 1 |
| pre_depth | number | | 1 |
| reflection | search | | 100.0 |
| shadow | caster | Determines what type of shadow casting the model will use. | 0 = None, 1 = Opaque, 2 = Alpha tested, 3 = Shadow only + opaque, 4 = Shadow only + alpha tested |
| shadow_bounding_box_offset | offset | | `(float, float, float)` with values from 9.0 to 100.0 |
| silhouette_type | number | | 0, 1 |
| stage_expansion_type | number | | 0, 1 |
| stage_ink_type | number | | 0, 1, 2, 3 |
| stage_sh_priority | number | | 0, 1, 2, 3, 4, 5 |
| stage_transition_type | number | Determines if a stage model temporarily blooms or not after the Stage Morph transition. | 0 = Enables bloom, 1 = ??? (identical to 0?), 2 = Disables bloom |
| stage_transition_visible | number | Determines a stage model's visibility with the Stage Morph transition background | 0 = Invisible, 1 = Visible |
| stage_visible_type | number | Determines a stage model's visibility with effect background appearences | 0 = Always visible, 1 = Disappears during animated effect backgrounds, 2 = Only visible during animated effect backgrounds |
| stencil_type | number | | 1, 2, 3, 4, 5, 6, 7, 8, 8, 10, 11, 12, 13, 14, 15 |
| this_light | action | | 0, False |
| this_light | color | | `float, float, float` with values from 0.0 to 1.0  |
| this_light | local_offset | | `float, float, float` with values from -100.0 to 0.0 |
| this_light | offset | | 0.0 |
| this_light | radius | | 9.0 to 100.0 |
| version | number | Identifies the XMB file format version. Changing it has no effect | 2 |

# lod.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| Mesh | parent | | mesh names, ex: armhair |
| Mesh | value | | mesh names, ex: armhair_high |
| Motion | value | | 0 |
| Ratio | value | | values from 0.0 to 1.0 |
| text_node | _text | | no value? |
| Visibility | value | | true, false |

# effect_locator.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| entry | effect_name |  | names starting with STG_ |
| entry | expansion_type | | |
| entry | id | | 0 |
| entry | joint | | bone names |
| entry | model | | model names, ex: dc_reaper_set |
| entry | pos | | `float, float, float` with values from -1000.0 to 1000.0  |
| entry | rot | | `float, float, float` with angle values (probably radians) |
| entry | scale | | `float, float, float` with values from 0.0 to 200.0 |
| entry | type | | 0, 1 |
| entry | user_data | Some sort of bit flags? | 0, 1, 2, 3, 4, 5, 65536, 196608 |

# area_light.xmb
| Entry | Attribute | Description | Values |
| --- | --- | --- | --- |
| entry | col_ceiling | | `float, float, float` with values from 0.0 to 3.0 |
| entry | col_ground | | `float, float, float` with values from 0.0 to 1.0|
| entry | id | | names, ex: groundGreen_01, red_17 |
| entry | pos | | `float, float, float` with values from -400.0 to 400.0 |
| entry | rot | | `float, float, float` with angle values (probably radians) |
| entry | scale | | `float, float, float` with values from 0.0 to 200.0 |
