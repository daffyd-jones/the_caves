## crates/drawers

_crate_

- envinter that contains an item enum
  - `EnvInter::Crate(Items::HealthPotion)`

- when player interacts with crate menu is provided that shows item,desc,value?
  - player can take or leave
  - if taken: Item is populated and added to Inventory/removed from envinter
  - else: envinter is left as-is

_drawer/cabinet_

- envinter that contains array of item enums
  - `EnvInter::Drawer([Items::Apple, Items::HealthPotion, Items::Rock])`

- same process as crate, with the addition of selcting items to grab or all

## Documents

- struct that contains document content

```rust
enum DocType {
  Book,
  Enscription,
  PaperScrap
}

struct Document {
  dtype: DocType,
  pos: (usize, usize),
  title: String,
  content: String
  icon: (char, Color)
}
```

```rust
impl Document {
  pub fn new_book(pos: (usize, usize), title: String, content: String) -> Self {
    Self {
      dtype: DocType::Book,
      pos,
      title,
      content,
      icon: ('🕮', Color::Green)
    }
  }
}

let book = Document::new_book((0, 0), "Title".to_string(), "This is a bunch of text that the player can read".to_string());

//----- 🕮 𜸂 ≚ ⋿  

impl Document {
  pub fn new_enscription(pos: (usize, usize), title: String, content: String) -> Self {
    Self {
      dtype: DocType::Enscription,
      pos,
      title,
      content,
      icon: ('≋', Color::Green)
    }
  }
}

let enscription = Document::new_enscription((0, 0), "".to_string(), "This is a bunch of text that the player can read".to_string());

//----

impl Document {
  pub fn new_paper_scrap(pos: (usize, usize), title: String, content: String) -> Self {
    Self {
      dtype: DocType::PaperScrap,
      pos,
      title,
      content,
      icon: ('⎎', Color::Green)
    }
  }
}

let paper_scrap = Document::new_paper_scrap((0, 0), "".to_string(), "This is a bunch of text that the player can read".to_string());

//----- ⎎ 𜲌 ⏥
```

- could also have enscriptions as envinter if it is done with an enum
- this would retrieve content from assets reactively rather than contained in the world
- `EnvInter::Document(DocEntity::ContentId)`

Steps

- player interacts with document envinter
- in environment_interactions `EnvInter::Document()` leads to asset retrieval func
- inner `DocEntity::ContentId` is used to retrieve content for string
