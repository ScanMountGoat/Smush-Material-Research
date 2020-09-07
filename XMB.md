# model.xmb
| Node | Property | Description / Values |
| --- | --- | --- |
| model | type | Determines what model type the model.xmb file is for. <br> effect = ??? <br> effect_main = ??? (Smash 4 leftover?) (fighter/pikachu/model/dengeki/c00/) <br> effect_near = ??? (Smash 4 leftover?) (fighter/pikachu/model/kaminari/c00/) <br> fighter = ??? <br> stage = ???
| version | number | Identifies the XMB file format version. Changing it has no effect. <br> 2 = Little-endian |
| shadow | caster | Determines what type of shadow casting the model will use. <br> 0 = None <br> 1 = Opaque <br> 2 = Alpha tested <br> 3 = Shadow only; opaque <br> 4 = Shadow only; alpha tested |
| lightset | number | Determines which light set to use from the lighting file. Light set type may be determined by the model type.
| draw_range | number | ???
| drawing_order | number | ??? <br> Values range from 0-255
| this_light | ??? | ???
| compress_type | number | ??? <br> 0 = ??? <br> 1 = ??? (fighter/gamewatch/model/body/c00/) <br> 2 = ??? (fighter/gamewatch/model/oil/c00/) <br> 4 = ??? (fighter/gamewatch/model/breath/c00/) <br> 5 = ??? (fighter/kirby/model/copy_gamewatch_fitkirby/c00/)
| stage_visible_type | number | Determines a stage model's visibility with effect background appearences. <br> 0 = Always visible <br> 1 = Disappears during animated effect backgrounds <br> 2 = Only visible during animated effect backgrounds
| mirror | number | ??? <br> 0x0000 = ???
| instancing | number | Enables geometry instancing for a model? <br> 0 = No instancing? <br> 1 = Instancing?
| aurora_vision | number | Related to jumbotrons or other screen projections. <br> 0 = Disabled? <br> 1 = Enabled?
| stage_transition_type | number | Determines if a stage model temporarily blooms or not after the Stage Morph transition. <br> 0 = Enables bloom <br> 1 = ??? (identical to 0?) <br> 2 = Disables bloom
| stage_transition_visible | number | Determines a stage model's visibility with the Stage Morph transition background. <br> 0 = Invisible <br> 1 = Visible
| bounding_scale | number | ???
| poison | number | ??? <br> 0 = ??? <br> 1 = ???
| stage_expansion_type | number | ??? (has a value of 1 on some fighter model types) <br> 0 = ??? <br> 1 = ??? (stage/animal_city/battle/model/main_ring_set/)
| stage_sh_priority | number | ??? <br> 0 = ??? <br> 1 = ??? (stage/75m/normal/model/stc_stgdonkey_elevator_set/) <br> 2 = ??? (stage/75m/normal/model/stc_chikei_set/) <br> 3 = ??? (stage/wreckingcrew/normal/model/stc_hashigo_floor_set/) <br> 4 = ??? (stage/bonusgame/normal/model/sbx0000_set/) <br> 5 = ??? (stage/bonusgame/normal/model/blackhole_gp_set/)
| is_multi_sh | number | ??? <br> 0 = ??? <br> 1 = ??? (stage/ff_midgar/normal/model/dyr_ring_set/)
| check_material | number | ??? <br> 0 = ???
| stencil_type | number | Used for predetermined stencil operations? <br> 1 = ??? (common value for fighter model types) <br> 2 = ??? (common value for stage model types) <br> 3 = ??? (item/pacmanapple/model/body/c00/) <br> 4 = ??? (common value for effect model types) <br> 5 = ??? (assist/andross/model/body/c00/) <br> 6 = ??? (stage/mother_magicant/normal/model/s25_sake_sky/) <br> 7 = ??? (stage/mother_magicant/normal/model/s25_myhome/) <br> 8 = ???  (assist/flyandhand/model/body/c00/) <br> 9 = ??? (fighter/buddy/model/entrywipe/c00/) <br> 10 = ??? (stage/wario_madein/normal/model/stgmadein04a/) <br> 11 = ??? (stage/tantan_spring/normal/model/near_ceiling_set/) <br> 12 = ??? (stage/tantan_spring/normal/model/near_silhouette_set/) <br> 13 = ??? (effect/system/common/model/m_cmnflyingplate/c00/) <br> 14 = ??? (fighter/samus/model/gbeam/c00/) <br> 15 = ??? (stage/pac_land/normal/model/stc_s00_set/)
| stage_ink_type | number | ??? <br> 0 = ??? <br> 1 = ??? (stage/streetpass/battle/model/main_ring_set/) <br> 2 = ??? (stage/pac_land/battle/model/main_ring_set/) <br> 3 = ??? (stage/poke_unova/normal/model/s13_e/)
| force_opaque | number | ??? <br> 0 = ??? <br> 1 = ??? (fighter/bayonetta/model/body/c00/)
| shadow_bounding_box_offset | offset | ???
| silhouette_type | number | ??? <br> 0 = ??? <br> 1 = ??? (fighter/koopag/model/body/c00/)

# lod.xmb
# effect_locator.xmb
# area_light.xmb
/stage/kirby_cave/battle/render/area_light.xmb<br>
/stage/kirby_cave/normal/render/area_light.xmb<br>
/stage/mario_galaxy/battle/render/area_light.xmb<br>
/stage/mario_galaxy/normal/render/area_light.xmb
