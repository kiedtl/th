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

                // is there a mob here
                if let Some(mob_id) = tile.mobs {
                    assert!(st.dungeon.mobs.contains_key(&mob_id));
                    let mob = &st.dungeon.mobs[&mob_id].clone();
                    mob_tick(st, mob, &mobs[&mob.from_mob_template], rng, lvl, x, y);
                }
            }
        }
    }
}

pub fn mob_tick<R>(st: &mut State, _mob: &Mob, template: &MobTemplate,
    rng: &mut R, lvl: usize, x: usize, y: usize)
where
    R: Rng
{
    if rng.gen_range(0, 100) > template.movement.chance_of_movement() {
        return; // nope
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
            return;
    }

    st.dungeon.move_mob(lvl, cur_pos.as_yx(), lvl, new_pos, true).unwrap();
}

pub fn player_tick(st: &mut State) {
    let player = st.dungeon.player;
    let player_y = player.coords.0;
    let player_x = player.coords.1;
    let player_mob_id = st.dungeon.at(player.level, (player_y, player_x))
        .mobs.unwrap();
    let player_mob = st.dungeon.mobs.get_mut(&player_mob_id).unwrap();

    let map_width = st.dungeon.levels[player.level].width;
    let map_height = st.dungeon.levels[player.level].height;

    // update the FOV
    let mut fov = FovRecursiveShadowCasting::new();
    let mut map = MapData::new(map_width, map_height);

    let starty = 0;
    let endy   = map_height;
    let startx = 0;
    let endx   = map_width;

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

    player_mob.fov = Vec::new();
    for y in startx..endy {
        for x in startx..endx {
            if map.is_in_fov(x, y) {
                let coord = Coord::from((y, x));
                let tile = st.dungeon.levels[player.level].d[y][x].clone();

                // add to field of vision
                player_mob.fov.push(coord);

                // add to memory, if
                // 1) this square hasn't been seen before
                // 2) or it has been seen before, but the tile has changed
                if !player_mob.memory.contains_key(&coord) ||
                    player_mob.memory.get(&coord).unwrap() != &tile {
                        if player_mob.memory.len() < MAX_PLAYER_MEMORY {
                            player_mob.memory.insert(coord, tile);
                        }
                }
            }
        }
    }
}
