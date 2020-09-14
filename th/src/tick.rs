use crate::state::*;
use lib::coord::*;
use rand::prelude::*;
use lib::dun_s1::*;
use lib::mob::*;
use doryen_fov::{
    FovAlgorithm,
    FovRecursiveShadowCasting,
    MapData
};
use std::collections::HashMap;

const PLAYER_VIEW_RADIUS: usize = 5;
const MAX_PLAYER_MEMORY: usize = 1024 * 1024 * 1024;
const MOB_VISION: usize = 3;

pub fn mobs_tick<R>(st: &mut State, mobs: &HashMap<String, MobTemplate>, rng: &mut R)
where
    R: Rng
{
    for lvl in 0..st.dungeon.levels.len() {
        for y in 0..st.dungeon.levels[lvl].height {
            for x in 0..st.dungeon.levels[lvl].width {
                if (y, x) == st.dungeon.player.coords {
                    continue;
                }

                let tile = st.dungeon.at(lvl, (y, x));

                // is there even a mob here
                let mob: &Mob;
                let mobtemplate: &MobTemplate;
                if let Some(mob_id) = tile.mobs {
                    assert!(st.dungeon.mobs.contains_key(&mob_id));
                    mob = &st.dungeon.mobs[&mob_id];
                    mobtemplate = &mobs[&mob.from_mob_template];
                } else {
                    continue;
                }

                // will the mob move on this round?
                if rng.gen_range(0, 100) > mobtemplate.movement.chance_of_movement() {
                    continue; // nope
                }

                let cur_pos = Coord::from((y, x));

                // get random direction
                let mut new_pos = cur_pos.as_yx();
                for _ in 0..8 {
                    new_pos = cur_pos.neighbor_in_direction(rng.gen())
                        .clamp_x(st.dungeon.levels[lvl].width)
                        .clamp_y(st.dungeon.levels[lvl].height)
                        .as_yx();
                    if st.dungeon.at(lvl, new_pos).tiletype != TileType::Wall {
                            break;
                    }
                }

                if st.dungeon.at(lvl, new_pos).tiletype == TileType::Wall {
                        continue;
                }

                st.dungeon.move_mob(lvl, cur_pos.as_yx(),
                    lvl, new_pos, true).unwrap();
            }
        }
    }
}

pub fn player_tick(st: &mut State) {
    let player = &mut st.dungeon.player;
    let player_y = player.coords.0;
    let player_x = player.coords.1;

    let map_width = st.dungeon.levels[player.level].width;
    let map_height = st.dungeon.levels[player.level].height;

    // so first we update the FOV
    let mut fov = FovRecursiveShadowCasting::new();
    let mut map = MapData::new(map_width, map_height);

    let starty = 0;//player_y.saturating_sub(map_height / 2);
    let endy   = map_height;//clamp(player_y + (map_height / 2), 0, map_height);
    let startx = 0;//player_x.saturating_sub(map_width / 2);
    let endx   = map_width;//clamp(player_x + (map_width / 2), 0, map_width);

    for y in starty..endy {
        for x in startx..endx {
            let tile = st.dungeon.levels[player.level].d[y][x].tiletype;

            let is_transparent = if tile == TileType::Wall {
                false
            } else {
                true
            };

            map.set_transparent(x, y, is_transparent);
        }
    }

    map.clear_fov();
    fov.compute_fov(&mut map, player_x, player_y, PLAYER_VIEW_RADIUS, true);

    player.in_fov = Vec::new();
    for y in startx..endy {
        for x in startx..endx {
            if map.is_in_fov(x, y) {
                // add to field of vision
                player.in_fov.push((y, x));

                // add to memory, so it will be shown even
                // if not in field of vision
                if !player.memory.contains(&(y, x)) {
                    if player.memory.len() < MAX_PLAYER_MEMORY {
                        player.memory.push((y, x));
                    }
                }
            }
        }
    }
}
