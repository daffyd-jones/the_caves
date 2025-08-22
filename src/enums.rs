//enum
use crate::enemy::Enemy;
use crate::features::Feature;
use crate::item::Item;
use crate::npc::{BaseNPC, CommNPC, ConvNPC, ShopNPC, SpawnNPC, TaskNPC, TradeNPC};
use crate::puzzle::Puzzle;
use crate::settlement::Settlement;
use std::fmt;
// Define the Cell enum
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cells {
    Empty,
    Transparent,
    Grass1,
    Grass2,
    Grass3,
    TallGrass,
    Bramble1,
    Bramble2,
    Bramble3,
    Bramble4,
    Bush,
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
    Broken5,
    Broken6,
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
    Tile1,
    Tile2,
    Tile3,
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
    BsVR,
    BsVL,
    BsHD,
    BsHU,
    CurUL,
    CurUR,
    CurBL,
    CurBR,
    BknWV,
    BknWH,
    Bed,
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
    ChairRight1,
    ChairRight2,
    Firewood,
    FireSmoke,
    FireDiamond,
    FireTri,
    Stand1,
    Stand2,
    Stand3,
    StandBL,
    StandDL,
    StandBS,
    StandDS, //----
    CircleVL,
    CircleHex,
    CircleC,
    Drawers,
    Shelves,
    Vase,
    LadderV,
    LadderH,
    TickV,
    TickH,
    Tech1,
    Tech2,
    Tech3,
    Tech4,
    Tech5,
    Tech6,
    Tech7,
    Tech8,
    Tech9,
    Tech10,
    Tech11,
    Tech12,
    Tech13,
    Tech14,
    Tech15,
    Tech16,
    Tech17,
    Relic1,
    Alembic,
    OldWall1,
    OldWall2,
    OldWall3,
    OldWall4,
    OldWall5,
    CardTile1,
    CardTile2,
    CardTile3,
    CardTile4,
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
    Seasonal1,
    Seasonal2,
    Seasonal3,
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
    // weapons
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
    // wearable
    ShieldingPendant,
    AgilityPendant,
    StrengthPendant,
    // documents
    Scroll,
    Gold,
    Null,
}

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Items::Apple => write!(f, "Apple"),
            Items::HealthPotion => write!(f, "Apple"),
            Items::VitalityPotion => write!(f, "Apple"),
            Items::Antidote => write!(f, "Apple"),
            Items::LuckPotion => write!(f, "Apple"),
            Items::AgilityPotion => write!(f, "Apple"),
            Items::Salve => write!(f, "Apple"),
            Items::Dowel => write!(f, "Apple"),
            Items::WoodenBoard => write!(f, "Apple"),
            Items::IronSword => write!(f, "Apple"),
            Items::MetalScrap => write!(f, "Apple"),
            Items::EdibleRoot => write!(f, "Apple"),
            Items::Guts => write!(f, "Apple"),
            Items::Rock => write!(f, "Apple"),
            Items::BronzeClaymore => write!(f, "Apple"),
            Items::IronClaymore => write!(f, "Apple"),
            Items::SteelClaymore => write!(f, "Apple"),
            Items::BronzeLongsword => write!(f, "Apple"),
            Items::IronLongsword => write!(f, "Apple"),
            Items::SteelLongsword => write!(f, "Apple"),
            Items::BronzeGreatsword => write!(f, "Apple"),
            Items::IronGreatsword => write!(f, "Apple"),
            Items::SteelGreatsword => write!(f, "Apple"),
            Items::BronzeShortsword => write!(f, "Apple"),
            Items::IronShortsword => write!(f, "Apple"),
            Items::SteelShortsword => write!(f, "Apple"),
            Items::BasicStaff => write!(f, "Apple"),
            Items::PineStaff => write!(f, "Apple"),
            Items::WoodStaff => write!(f, "Apple"),
            Items::MapleStaff => write!(f, "Apple"),
            Items::OakStaff => write!(f, "Apple"),
            Items::BludgeonStaff => write!(f, "Apple"),
            Items::GemStaff => write!(f, "Apple"),
            Items::BronzeHeavyAxe => write!(f, "Apple"),
            Items::IronHeavyAxe => write!(f, "Apple"),
            Items::SteelHeavyAxe => write!(f, "Apple"),
            Items::BronzeLightAxe => write!(f, "Apple"),
            Items::IronLightAxe => write!(f, "Apple"),
            Items::SteelLightAxe => write!(f, "Apple"),
            Items::BronzePickAxe => write!(f, "Apple"),
            Items::IronPickAxe => write!(f, "Apple"),
            Items::SteelPickAxe => write!(f, "Apple"),
            Items::BronzePickHammer => write!(f, "Apple"),
            Items::IronPickHammer => write!(f, "Apple"),
            Items::SteelPickHammer => write!(f, "Apple"),
            Items::ShadowAxe => write!(f, "Apple"),
            Items::BronzeWarAxe => write!(f, "Apple"),
            Items::IronWarAxe => write!(f, "Apple"),
            Items::SteelWarAxe => write!(f, "Apple"),
            Items::LightArmour => write!(f, "Apple"),
            Items::MediumArmour => write!(f, "Apple"),
            Items::HeavyArmour => write!(f, "Apple"),
            Items::SmallWoodShield => write!(f, "Apple"),
            Items::LargeWoodShield => write!(f, "Apple"),
            Items::IronShield => write!(f, "Apple"),
            Items::SteelShield => write!(f, "Apple"),
            Items::ShieldingPendant => write!(f, "Apple"),
            Items::AgilityPendant => write!(f, "Apple"),
            Items::StrengthPendant => write!(f, "Apple"),
            Items::Scroll => write!(f, "Apple"),
            Items::Gold => write!(f, "Apple"),
            Items::Plants(_plants) => todo!(),
            Items::Null => todo!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Ord, PartialOrd)]
pub enum Month {
    Opal,
    Quartz,
    Jade,
    Bizmuth,
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
    VioletShadow,
    LampenFlower,
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
    Gold,
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
    ShopItem(ShopItem),
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
    Construction,
    Cauldron,
    Task(TaskType),
    Door(Door),
    ShopNPC(Shops),
    Herbalist,
    Hermit,
    TaskEnv(TaskEnv),
    WoodenHatch,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskEnv {
    BoardStartEntity,
    BoardGoalEntity,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum TaskType {
    Plot,
    RetrieveItem,
    PassMessage,
    PassItem,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Door {
    HOpen,
    HUnlocked,
    HLocked(u8),
    VOpen,
    VUnlocked,
    VLocked(u8),
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
    Guild,
    Obsidian,
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
    Clinic,
    Herbalist,
    Weapon,
    Armor,
    Consignment,
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ShopItem {
    Item(Item),
    Guild,
    Church,
    Clinic,
    Herbalist(Item),
    Weapon(Item),
    Armor(Item),
    Consignment(Item),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Settlement(Settlement),
    Puzzle(Puzzle),
    Feature(Feature),
    Null,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FeatureType {
    Field,
    Stream,
    Pond,
    Construction,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PuzzleType {
    Maze,
    Ruin,
    Teleport,
    Inverted,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DialogueTypes {
    None,
}
