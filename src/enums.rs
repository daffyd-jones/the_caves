//enum
use crate::enemy::Enemy;
use crate::features::Feature;
use crate::item::Item;
use crate::npc::{BaseNPC, CommNPC, ConvNPC, ShopNPC, SpawnNPC, TradeNPC};
use crate::puzzle::Puzzle;
use crate::settlement::Settlement;
// Define the Cell enum
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cells {
    Empty,
    Grass1,
    Grass2,
    Grass3,
    TallGrass,
    Bramble1,
    Bramble2,
    Bramble3,
    Bramble4,
    Dirt1,
    Dirt2,
    Dirt3,
    Moss,
    Rock,
    Wall,
    Wall2,
    Wall3,
    Wall4,
    Broken1,
    Broken2,
    Broken3,
    Broken4,
    Roots,
    ULCorner1,
    ULCorner2,
    ULCorner3,
    ULCorner4,
    ULCorner5,
    URCorner1,
    URCorner2,
    URCorner3,
    URCorner4,
    URCorner5,
    DLCorner1,
    DLCorner2,
    DLCorner3,
    DLCorner4,
    DLCorner5,
    DRCorner1,
    DRCorner2,
    DRCorner3,
    DRCorner4,
    DRCorner5,
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
    DblBracedGate, //-------
    BracedGate,
    Arch,
    Bricks,
    Crops,
    SmallCampfire,
    Campfire,
    Table,
    Chair,
    Firewood,
    Tent,
    Jar,
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
    Herbalist,
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Items {
    HealthPotion, // +10 health
    VitalityPotion,
    Antidote,
    LuckPotion,
    AgilityPotion,
    Salve,
    Dowel,
    WoodenBoard,
    IronSword,
    MetalScrap,
    Apple,
    EdibleRoot,
    Guts,
    Rock,
    BronzeClaymore,
    IronClaymore,
    SteelClaymore,
    BronzeLongsword,
    IronLongsword,
    SteelLongsword,
    BronzeGreatsword,
    IronGreatsword,
    SteelGreatsword,
    BronzeShortsword,
    IronShortsword,
    SteelShortsword,
    BasicStaff,
    PineStaff,
    WoodStaff,
    MapleStaff,
    OakStaff,
    BludgeonStaff,
    GemStaff,
    BronzeHeavyAxe,
    IronHeavyAxe,
    SteelHeavyAxe,
    BronzeLightAxe,
    IronLightAxe,
    SteelLightAxe,
    BronzePickAxe,
    IronPickAxe,
    SteelPickAxe,
    BronzePickHammer,
    IronPickHammer,
    SteelPickHammer,
    ShadowAxe,
    BronzeWarAxe,
    IronWarAxe,
    SteelWarAxe,

    LightArmour,
    MediumArmour,
    HeavyArmour,

    SmallWoodShield,
    LargeWoodShield,
    IronShield, // +10 defence
    SteelShield,

    //
    Plants(Plants),
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum Month {
    Opal,
    Quartz,
    Jade,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum ToggleState {
    PlayerTraits(PlayerTraits),
    NPCIntros(NPCIntros),
    Plants(Plants),
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum PlayerTraits {
    Poisoned,
    Agility,
    Vitality,
    Invisible,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum NPCIntros {
    Herbalist,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum ExpType {
    Attack,
    Damage,
    Defence,
    Luck,
    Trading,
    Lockpicking,
    Navigation,
    Herbalism,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum Plants {
    Moss,
    LuminousMushroom,
    LichenousGrowth,
    VineBulb,
    LampenPetals,
    LuckyClover,
    Shroom,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum Equip {
    Weapon,
    Shield,
    Armour,
    Wearing,
    Hands,
    Head,
    Torso,
    Feet,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum ItemEffect {
    Health,
    Attack,
    Damage,
    Defence,
    Luck,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Enemies {
    Golem,
    CrazedExplorer,
    Goblin,
    Slime,
    Snake,
    Spider,
    Bandit,
    Ghoul,
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
    TradeNPC,
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NPCWrap {
    CommNPC(CommNPC),
    ConvNPC(ConvNPC),
    SpawnNPC(SpawnNPC),
    ShopNPC(ShopNPC),
    TradeNPC(TradeNPC),
    BaseNPC(BaseNPC),
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AniType {
    Player,
    Area,
    Char,
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
    Dead,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EncMode {
    Auto,
    Manual,
    Quick,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EncResult {
    Win,
    Lose,
    Cont,
    Null,
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
pub enum EnvInter {
    Records,
    Clinic,
    GuildPost,
    ChurchPost,
    Cauldron,
    LockedDoor,
    Door,
    Herbalist,
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
    Null,
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
pub enum NodeType {
    Settlement,
    Puzzle,
    Feature,
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
    Feature(Feature),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FeatureType {
    Field,
    Stream,
    Pond,
    Ruin,
    AbandonedShack,
    AbandonedSettlement,
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
    Inverted,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DialogueTypes {
    None,
}
