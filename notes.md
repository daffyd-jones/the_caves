# todo

- Trade Npcs
- polish trade functionality
- Puzzles
- Teleport
- Maze X
- Inverted
- Compass ~

- Fix Loc spawn/management ~
- Remove items when wall
- ShopNPCs X
- cursor limit
- enchantments
- notebook
- add with SpawnNPC
  - update with Puzzle
- add with Settlement

**animations**:

- Fight enemy design
- Equip icons

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

# Equipment

## Weapons

Add buff to damage
Some add buff to attack

## Torso

Add buff to defence
Some add buff to attack

## Helmet

Add buff to defence

## Boots

Add buff to defence
some add buff to attack

## Shield

Add buff to defence

## Hands

Some add buff to defence
Some add buff to attack
Some add buff to damage

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

# Potential Mechanics

- Bartering: The more a character trades with NPCs, the better deals the player gets
- Herbalism: The player can learn more about the various fungi in the caves
  - can be used to create Health Potions, Salves, Anti-toxin, other buff potions (Agility Potion)
  - can be developed by finding fungi in the caves
  - in the caves//settlements there are herbalists that will identify the fungi you find
  - they will also tell you what you can make with the various fungi types
- Medicine: The player can develop the ability to heal themselves
  - the player can talk to some of the cliic managers and receive first aid education
  - as the player visits different clinics they can upgrade their abilities to heal themselves
-

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

## Bartering

- Each time the player trades with a TradeNPC they gain more xp
- The higher the xp, the better deals they get when buying and selling
- This can be used to increasingly gain wealth throughought the game.
