//enum
use crate::item::Item;
use crate::enemy::{Enemy};
use crate::puzzle::{Puzzle};
use crate::settlement::{Settlement};
use crate::npc::{BaseNPC, CommNPC, ConvNPC, ShopNPC, SpawnNPC};

// Define the Cell enum
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cells {
    Empty,
    Grass1,
    Grass2,
    Dirt1,
    Dirt2,
    Rock,
    Wall,
    Floor,
    Floor2,
    Tunnel,
    Water,
    MwH,
    MwV,
    MwVL,
    MwVR,
    MwHU,
    MwHD,
    MwUL,
    MwUR,
    MwDL,
    MwDR,
    MwCR,
    SwH,
    SwV,
    SwVL,
    SwVR,
    SwHU,
    SwHD,
    SwUL,
    SwUR,
    SwDL,
    SwDR,
    SwCR,
    Cong,
    Deg,
    Mult,
    Ced,
    Diae,
    Inter,
    Blsq,
    VBrk,
    PlMin,
    Exup,
    SmZer,
    BZer,
    Cop,
    NPCM,
    LBrce,
    RBrce,
    LParen,
    RParen,
    GenCur,
    Enemy,
    NPC,
    Item,
    Log,
    Clinic,
    GPost,
    CPost,
    Null
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Items {
    HealthPotion, // +10 health
    Salve,
    Dowel,
    WoodenBoard,
    IronShield, // +10 defence
    IronSword,
    MetalScrap,
    Apple,
    EdibleRoot,
    BugBits,
    Rock,
    //
    MagicRing,
    WeirdCloak,
    PhoenixFeather,
    Antidote,
    ShadowBoots,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum Equip {
    Weapon,
    Shield,
    Hands,
    Head,
    Torso,
    Feet,
    Null
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum ItemEffect {
    Health,
    Attack,
    Damage,
    Defence,
    Null
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Enemies {
    Golem,
    CrazedExplorer,
    GoblinMan,
    Slime,
    Bug,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NPCs {
    CommNPC,
    ConvNPC,
    QuestNPC,
    ShopNPC,
    SpawnNPC,
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NPCWrap {
    CommNPC(CommNPC),
    ConvNPC(ConvNPC),
    SpawnNPC(SpawnNPC),
    ShopNPC(ShopNPC),
    BaseNPC(BaseNPC),
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GUIMode {
    Bug,
    Normal,
    Interact,
    Inventory,
    Notes,
    Map,
    NPC,
    Fight,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameMode {
    Menu,
    Play,
    Interact(InterSteps),
    Fight(FightSteps),
    Converse,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InterSteps {
    AdjOpt,
    IntOpt,
    Feedback,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FightSteps {
    Open,
    Enemy,
    Player,
    Message,
    Null,
}

#[derive(Clone, Debug)]
pub enum Interactable {
    Item(Item),
    ShopItem(Item),
    NPC(NPCWrap),
    Enemy(Enemy),
    EnvInter(EnvInter),
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EnvInter{
    Records,
    Clinic,
    GuildPost,
    ChurchPost,
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub enum InterOpt {
    Item(ItemOpt),
    // Enemy(EnemyOpt),
    // NPC,
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub enum ItemOpt {
    PickUp,
    Drp,
    Use,
    Equip,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub enum EncOpt {
    Attack,
    UseItem,
    Dodge,
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Settle {
    Small,
    Med,
    Large,
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Shops {
    Item,
    Guild,
    Church,
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Settlement(Settlement),
    Puzzle(Puzzle),
    Null,
}



#[derive(Clone, Debug, PartialEq)]
pub enum CompMode {
    Search,
    Location,
}


#[derive(Clone, Debug, PartialEq)]
pub enum PuzzleType {
    Maze,
    Teleport,
    Inverted
}


