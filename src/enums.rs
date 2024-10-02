//enum
use crate::item::Item;
use crate::enemy::{Enemy};
use crate::npc::{BaseNPC, CommNPC, ConvNPC, QuestNPC, ShopNPC};

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
    Null
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Items {
    HealthPotion, // +10 health
    IronSheild, // +10 defence
    IronSword,
    MagicRing,
    WeirdCloak,
    PhoenixFeather,
    Salve,
    Antidote,
    ShadowBoots,
    BugBits,
    MetalScrap,
    Apple,
    EdibleRoot,
    Rock,
    Null,
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Enemies {
    CrazedExplorer,
    GoblinMan,
    Bug,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NPCs {
    OldMan,
    Explorer,
    LostItemQuest,
    LostExplorer,
    Cultist,
    CommNPC,
    ConvNPC,
    QuestNPC,
    ShopNPC,
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NPCWrap {
    CommNPC(CommNPC),
    ConvNPC(ConvNPC),
    QuestNPC(QuestNPC),
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
    NPC(NPCWrap),
    Enemy(Enemy),
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InterOpt {
    Item(ItemOpt),
    // Enemy(EnemyOpt),
    // NPC,
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemOpt {
    PickUp,
    Drp,
    Use,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    Weapon,
    Armour,
    Enchantments,
    ExplorersGuild,
    CultistChurch,
    Clinic,
    Null,
}
