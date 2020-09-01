# TODO

## th (the actual game)
- very basic monster AI implementation
  - if next to player; attack, if not, stay still
    - if heavily wounded, run away
    - once recovered of wounds, come back

## mv (mapview)
- diag. movement with YUBN keys
- don't hardcode keybindings help
- show more information for materials (e.g. hardness)
- show information for mobs on tiles
- show information for items on tiles

## mg (mapgen)

- Implement BSP algorithm (for workshops and stuff)
- Drunkard: allow diagonal movement (NE, NW, SE, SW)
- Maze: randomly draw mazes until a certain amount of the map is filled
- Mineral mapgen:
  - add some new stones! (metamorphic: gneiss, marble, quartzite, slate,
    soapstone), (igneous extrusive: dacite), (sedimentary: sandstone,
    shale, mudstone, limestone, dolomite, chalk), (ores: copper ore, more
    iron ore, silver ore, gold ore, platinum ore, iridium ore, lead ore,
    uranium ore)
    - additionally: alabaster (gypsum), alunite (igneous extrusive,
      kaolinte), bauxite (sedimentary), kaolinite (sedimentary), borax
      (gypsum), calcite (limestone, marble), graphite (gneiss, quartzite,
      marble), gypsum (sedimentary), kimberlite (gabbro), mica (metamorphic,
      granite), olivine (gabbro), orthoclase (igneous intrusive,
      metamorphic), saltpeter (sedimentary), serpintine (olivine), talc
      (dolomite) plus some gems (diamonds, rubies, sapphire, opals,
      zircons, turquoise, jaspers, pyrite)
- Features/Items mapgen:
  - define Item struct
    - weapons
    - armor
    - scrolls (prayers, maps, junk, snippets from LOTR)
    - potions (narcotics, alcohol, poison, steroids, drugs)
    - id cards
  - create stockpiles
  - stairs and portals
