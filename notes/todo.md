## gameplay

### Hi


- [ ] make experience handling/upgrading
  - [+] implement xp_inc
  - [ ] add xp_inc to interactions/encounters
  - [ ] add msg when lvl up

### Lo

- [ ] implement puzzles
  - [+] keys
  - [ ] door toggle

- [ ] fix big wall in map
- [ ] fix cornercell orientation

- [ ] Grading for weapons/enemies

- [ ] add more equips
  - [ ] unique weapons
  - [ ] buffed weapons
  - [ ] cursed weapons

- [ ] Task weapons
  - starts debuff/weak => better once lifted
  - kill x enemies
  - bring to place
  - assemble set
  - combined item: 5 => 1

- [ ] implement timed effects/buffs
  - [ ] venom - slime, spider, snake, bug
  - [ ] shroom - take .25 HP, XP+: if below .75 HP => insta death, or .5 HP
  - [+] vitality potion
  - [+] agility potion
  - [+] strength potion
  - [+] luck potion
  - [ ] fire shield - lava areas

- [ ] tasks
  - [+] give item
  - [ ] bring item
  - [ ] deliver message
  - [ ] lost item
  - [ ] defeat enemy => item
  - [ ] monster hunter
  - [ ] lost convoy


- [ ] capsite
 - [ ] add item
 - [ ] tent envinter - take down, sleep
 - [ ] health
 - [ ] 1 a day

- [ ] lockpicking activity

- [ ] small map
  - [ ] switch map - store current, swap to small
  - [ ] put small in temp with wall
  - [ ] return to big map

- [ ] consignment box
  - [ ] need to talk to person at guild_settle to get access

- [ ] enchantments
  - [+] implement enchanted equip items
  - [+] finish player effects
    - [+] add effect check/toggle
    - [+] add item>toggle effect
    - [+] add timer for effects

- [ ] add notes for settlements, cave knowledge, discoveries (puzzles, intresting features)
  - [+] add get_info for settlement
  - [+] add settle info to settlements notes
  - [ ] add info for lore env inters
  - [ ] add get_info for puzzles

- [ ] implement ascii
  - [ ] implement variations for npcs
  - [ ] implement variations for items

## entities

- [ ] enemy grading
  - [ ] implement enemy_stats in Stats
  - [ ] implement player_stats check
  - [ ] check enemy rating when spawning
  - [ ] update enemy_stats when player levels or hits radius threshold
- [ ] weapons grading - availability vs buff

## admin

- [ ] CLEAN UP GUI
- [ ] menus - main/ingame/death
  - [+] main menu
  - [+] ingame menu
  - [ ] death menu
- [ ] save/load
- [ ] test area animate

## Done


- [+] treasure chest/crates
  - [+] icon
  - [+] enum
- [+] cursor limit
- [+] fix wall corners
- [+] fix collisions
- [+] implement plants
  - [+] implement constructors
  - [+] implement placement
  - [+] implement cauldron
  - [+] implement herbalist
    - [+] create EnvInter
    - [+] implement convo
    - [+] implement plant identification
    - [+] implement EnvInter toggle
- [+] ShopNPC -> EnvInter
- [+] fix enemy ascii
- [+] re-arrange notes
- [+] ingame menu  
- [+] fixed inventory bug
- [+] 

- [+] implement features - creation/nodemap placement
  - [+] nodemap creation
  - [+] feature creation
  - [+] add feature types
    - [+] Abandoned Shack
    - [+] Ruins
    - [+] Pond
    - [+] Field
    - [+] Stream
    - [+] Construction
  - [+] implement Hermit
    - hermit overlay in features

- [+] finish hermit
  - [+] EnvInter convo
  - [+] Shop interaction

- [+] finish item task
  - [+] give item to goal
  - [+] populate complete task
  - [+] Steward convo + reward

- [+] implement Tasks (EnvInter) - for retrieving items (will require persistance, best for settles/features)
  - [+] create EnvInter
  - [+] implement convos
  - [+] create Task
  - [+] implement task creation
  - [+] implement task completion
  - [+] implement completion convo/reward
