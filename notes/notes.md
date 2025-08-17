# todo

## gameplay

- [ ] implement Tasks (EnvInter) - for retrieving items (will require persistance, best for settles/features)
  - [+] create EnvInter
  - [+] implement convos
  - [+] create Task
  - [ ] implement task creation
  - [ ] implement task completion
  - [ ] implement completion convo/reward

- [ ] small map
  - [ ] switch map - store current, swap to small
  - [ ] put small in temp with wall
  - [ ] return to big map

- [ ] consignment box
  - [ ] need to talk to person at guild_settle to get access

- [ ] make experience handling/upgrading
  - [+] implement xp_inc
  - [ ] add xp_inc to interactions/encounters
  - [ ] add msg when lvl up

- [ ] enchantments
  - [ ] implement enchanted equip items
  - [ ] finish player effects
    - [ ] add effect check/toggle
    - [ ] add item>toggle effect
    - [ ] add timer for effects

- [ ] add notes for settlements, cave knowledge, discoveries (puzzles, intresting features)
  - [+] add get_info for settlement
  - [+] add settle info to settlements notes
  - [ ] add info for lore env inters
  - [ ] add get_info for puzzles

- [ ] implement ascii
  - [ ] implement variations for npcs
  - [ ] implement variations for items

- [ ] implement features - creation/nodemap placement
  - [+] nodemap creation
  - [+] feature creation
  - [ ] add feature types
    - [+] Abandoned Shack
    - [+] Ruins
    - [+] Pond
    - [+] Field
    - [+] Stream
    - [+] Construction
  - [ ] implement Hermit
    - hermit overlay in features

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
  - [ ] ingame menu
  - [ ] death menu
- [ ] save/load
- [ ] test area animate

## Done

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

# Seasons

- different plants, dialogue, enemies

## Plants

- colors for plant icons

**Damp season** - Opal

- moss more likely

**Drying season** - Quartz

- flowers more likely

**Dry season** - Jade

- clovers more likely

**Dampening season** - Bizmuth

- lichenous growth more likely

## Map

- decorations
- Seasonal features
- changed grass, etc icons

## Dialogue

**Damp season**

- talking about plants
- talking about Obsidian rituals
- guild event
- likes dislikes
- economy

# Enemies

Snakes ʑ
slimes ǚ
spiders ẅ
bandit Ồ
goblin ớ
ghoul ή

**animations**:

- Fight enemy design
- Equip icons

# Features

added map elements that are interacted with similar to settlements
may rarely have one npc or a few items

**types**:

- fields, grass features
- stream
- pond/lake
- ruin
- shack
- abandoned town/farm

# Puzzles

## Maze

- use carve algo to create map X
- place item on one side X
- remove outer wall on other side X

## Teleport

- generate node graph of steps
- use graph to chain the teleport cells
- place walls between connection points

## Inverted

- use carve to make maze
- set some

# Compass

- fix menu
- switch heading
- scroll list

# Notebook

## Settlements

- settlements that are found are added to this panel
- profile is populated with content from generated settlement
- Location relative to origin, closest settlement
- Shops that are present:
  - Shop names
  - ShopNPC names
- resident NPC names
- population
- possible comment about feature?

## Conversations

- conversations had with CommNPC, and ConvNPC entities are saved with their name

```
CommNPC
--

  Kevin: "This is a thing that the player said."

--
```

```
ConvNPC
--

  Kevin: "This is a thing that the NPC said."

  You: "This is the response the player chose."

  Kevin: "This is the thing that the NPC said in response to your choice."

  You: "This is the next response the player picked."

--
```

## Knowledge

- information about the caves and general lore is put here
- provided as entries with headers

```
  This is a header

  This is an entry paraphrasing the knowledge gained by an encounter.
  The length of an entry should be informative and concise.

  - The entry can also include a list
  - The list should be used for specific details
  - The list should be at most around 5-7
```

## Discoveries

- entries for spawned puzzles are provided here.
- when the player talks to a SpawnNPC the entry is added
- the entry contains a description of the spawned location
- the entry should be a paraphrasing of the content of the SpawnNPC conversation

```
This is a description of the spawned location that contains the
same information that was provided in the conversation with the
SpawnNPC. This contains a general description of the location
and the position relative to the current location, possibly
the position relative to the closest settlement
```

# Equipment

## Weapons

Add buff to damage
Some add buff to attack

## Armour

Add buff to defence
Some add buff to attack

## Shield

Add buff to defence

## Enchanted Items

Provide buffs and resistances

items such as:

- amulets
- rings
- random objects

## Cursed items

some enchanted items are cursed and have negative effects in addition to their positive ones

eg: +10 to damage, but -5 to defence

# Experience

Can be accumulated to increase capabilities

## Fighting stats

- Attack: gain xp when attack roll hits
- Defence: gain xp when enemy attack roll misses
- Damage: gain xp when damage roll is > max/2

# Non-combat stats

- Trading: The more a character trades with NPCs, the better deals the player gets
- Herbalism: The player can learn more about the various fungi in the caves
  - can be used to create Health Potions, Salves, Anti-toxin, other buff potions (Agility Potion)
  - can be developed by finding fungi in the caves
  - in the caves//settlements there are herbalists that will identify the fungi you find
  - they will also tell you what you can make with the various fungi types
- Medicine: The player can develop the ability to heal themselves
  - the player can talk to some of the cliic managers and receive first aid education
  - as the player visits different clinics they can upgrade their abilities to heal themselves
- Luck: Increases the lkelyhood of having better drops, improves luck on rolls
- Navigation: Improves ability to move through the caves, and know where places are.

## Lockpicking

- some doors have locks
- doors have security levels 0-31
- player earns xp for success
- player must start with easier doors, etc.

door locations:

- features
- settlements
- puzzles?

## Luck

- gain xp by having winning rolls
- luck potions are rare loot
- pray at cave temple
- gather `x` for cave cultists

## Navigation

- gain xp by finding locations: {settlements, puzzles, features}

Levels:

1. basic directional compass to closest settlement - maybe puzzle
2. history
3. distance gauge
4. xy distance
5. small map
6. large map

## Herbalist

- player can find plants/fungi in the caves
- settlements have herbalists that can identify plant that the player brings them
- known plants can be used to make potions and other items

**Luminous Mushroom**:

- have a low-med probability of being spawned
- can be used to make health potions

**Moss**:

- have a med probability of being spawned
- can be used to make salves

**Lichenous Growth**:

- have a med probability of being spawned
- can be used to make xxx potions

**Lucky clover**:

- have a med probability of being spawned
- can be used to make xxx potions

**Shroom**:

- causes player to enter altered state
  - they do not feel pain and are bold when attacking
  - the damage will be felt when the effect wears out
- takes 25% damage
- +5 to attack
- the outstanding damage is accumulated in a var, and is applied at once when the effect ends
- if the amount is more than the player's max health, this will cause instadeath
- if the player is at their max health, it is reduced to half
- this the only way to survive an accumulation higher than max health

## Trading

- Each time the player trades with a TradeNPC they gain more xp
- The higher the xp, the better deals they get when buying and selling
- This can be used to increasingly gain wealth throughought the game.

## plants

- plants that can be used for herbalism are placed
- there is roll done to determine which plants are generated.
- place the plants similar to items, but in groupings
- there could be roll for plant being overgrown, causes plant to fill empty space.
