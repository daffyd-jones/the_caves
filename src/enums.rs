//enum
use crate::enemy::Enemy;
use crate::features::Feature;
use crate::item::Item;
use crate::npc::{BaseNPC, CommNPC, ConvNPC, ShopNPC, SpawnNPC, TaskNPC, TradeNPC};
use crate::puzzle::{Puzzle, PuzzleDoor, PuzzleKey};
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
    Book,
    PaperScrap,
    HealthPotion, // +10 health
    VitalityPotion,
    Antidote,
    LuckPotion,
    AgilityPotion,
    StrengthPotion,
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
            Items::Book => write!(f, "Book"),
            Items::PaperScrap => write!(f, "Paper Scrap"),
            Items::Apple => write!(f, "Apple"),
            Items::HealthPotion => write!(f, "Health Potion"),
            Items::VitalityPotion => write!(f, "Vitality Potion"),
            Items::Antidote => write!(f, "Antidote"),
            Items::LuckPotion => write!(f, "Luck Potion"),
            Items::AgilityPotion => write!(f, "Agility Potion"),
            Items::StrengthPotion => write!(f, "Strength Potion"),
            Items::Salve => write!(f, "Salve"),
            Items::Dowel => write!(f, "Dowel"),
            Items::WoodenBoard => write!(f, "Wooden Board"),
            Items::IronSword => write!(f, "Iron Sword"),
            Items::MetalScrap => write!(f, "Metal Scrap"),
            Items::EdibleRoot => write!(f, "Edible Root"),
            Items::Guts => write!(f, "Guts"),
            Items::Rock => write!(f, "Rock"),
            Items::BronzeClaymore => write!(f, "Bronze Claymore"),
            Items::IronClaymore => write!(f, "Iron Claymore"),
            Items::SteelClaymore => write!(f, "Steel Claymore"),
            Items::BronzeLongsword => write!(f, "Bronze Longsword"),
            Items::IronLongsword => write!(f, "Iron Longsword"),
            Items::SteelLongsword => write!(f, "Steel Longsword"),
            Items::BronzeGreatsword => write!(f, "Bronze Greatsword"),
            Items::IronGreatsword => write!(f, "Iron Greatsword"),
            Items::SteelGreatsword => write!(f, "Steel Greatsword"),
            Items::BronzeShortsword => write!(f, "Bronze Shortsword"),
            Items::IronShortsword => write!(f, "Iron Shortsword"),
            Items::SteelShortsword => write!(f, "Steel Shortsword"),
            Items::BasicStaff => write!(f, "Basic Staff"),
            Items::PineStaff => write!(f, "Pine Staff"),
            Items::WoodStaff => write!(f, "Wood Staff"),
            Items::MapleStaff => write!(f, "Maple Staff"),
            Items::OakStaff => write!(f, "Oak Staff"),
            Items::BludgeonStaff => write!(f, "Bludgeon Staff"),
            Items::GemStaff => write!(f, "Gem Staff"),
            Items::BronzeHeavyAxe => write!(f, "Bronze Heavy Axe"),
            Items::IronHeavyAxe => write!(f, "Iron Heavy Axe"),
            Items::SteelHeavyAxe => write!(f, "Steel Heavy Axe"),
            Items::BronzeLightAxe => write!(f, "Bronze Light Axe"),
            Items::IronLightAxe => write!(f, "Iron Light Axe"),
            Items::SteelLightAxe => write!(f, "Steel Light Axe"),
            Items::BronzePickAxe => write!(f, "Bronze Pick Axe"),
            Items::IronPickAxe => write!(f, "Iron Pick Axe"),
            Items::SteelPickAxe => write!(f, "Steel Pick Axe"),
            Items::BronzePickHammer => write!(f, "Bronze Pick Hammer"),
            Items::IronPickHammer => write!(f, "Iron Pick Hammer"),
            Items::SteelPickHammer => write!(f, "Steel Pick Hammer"),
            Items::ShadowAxe => write!(f, "Shadow Axe"),
            Items::BronzeWarAxe => write!(f, "Bronze War Axe"),
            Items::IronWarAxe => write!(f, "Iron War Axe"),
            Items::SteelWarAxe => write!(f, "Steel War Axe"),
            Items::LightArmour => write!(f, "Light Armour"),
            Items::MediumArmour => write!(f, "Medium Armour"),
            Items::HeavyArmour => write!(f, "Heavy Armour"),
            Items::SmallWoodShield => write!(f, "Small Wood Shield"),
            Items::LargeWoodShield => write!(f, "Large Wood Shield"),
            Items::IronShield => write!(f, "Iron Shield"),
            Items::SteelShield => write!(f, "Steel Shield"),
            Items::ShieldingPendant => write!(f, "Shielding Pendant"),
            Items::AgilityPendant => write!(f, "Agility Potion"),
            Items::StrengthPendant => write!(f, "Strength Pendant"),
            Items::Scroll => write!(f, "Scroll"),
            Items::Gold => write!(f, "Gold"),
            Items::Plants(_plants) => todo!(),
            Items::Null => write!(f, ""),
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
    Antidote,
    Agility,
    Vitality,
    Strength,
    Read,
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
    PuzzlePiece(PuzzlePiece),
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PuzzlePiece {
    PuzzleDoor(PuzzleDoor),
    PuzzleKey(PuzzleKey),
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
    Crate(Items),
    Cabinet([Items; 3]),
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskEnv {
    BoardStartEntity,
    BoardGoalEntity,
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskType {
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
    Hermit,
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
    Hermit(Item),
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
    Flip,
    KeyRuin,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DialogueTypes {
    None,
}
