// tasks.rs

use std::collections::HashMap;
use std::fs;

use rand::seq::SliceRandom;

use crate::enums::{Items, Location, ToggleState};
use crate::item::Item;
use crate::npc::{ConOpt, Convo, Stage};

// # task is defined in settlement/feature
//  $ location is built
//   + envinter is placed
//   + task is built
//    - type in loc chunk: Plot (predefined) || Other (built)
//    - content loaded/built
//    - given to settlement
//  $ location rendered - content active
//   + pass envinter w/ rest of state
//  $ player interacts w/ envinter
//   + get task
//   + do init convo
//   + if select yes
//    - end convo
//    - pass task to Tasks - pass envinter to Settle/Feature
//   + if no
//    - end convo
//
// # is guild task
//  $ pre: check num of task postings
// _$ pre: if not max: gen task to posted_tasks
//  $ player interacts with posting
//  $ move posting: posted_tasks > active_tasks
//
// # player visit goal loc
//  $ render envinter in self.env_inters
//
// # player interacts with envinter
//  $ passed to interactable
//  $ if Plot
//   - run plot dialogue
//   - set task/state toggle/s
//  $ if RetrieveItem
//   - pick up item
//   - set task/state toggle/s
//  $ if PassMessage
//   - run dialogue
//   - set task/state toggle/s
//   - if done: remove task
//   - else: task contains follow-up
//  $ if PassItem
//   - run dialogue
//   - remove item from inventory
//   - set task/state toggle/s
//   - if done: remove task
//   - else: task contains follow-up
//
// # if follow-up
//  $ if guild
//   + player talk to guild head
//   + give player reward
//  $ if other
//   + dialogue with npc
//   + if reward: give
//  $ Remove task

/*

req:
- needs start properties
 - npc pos, loctype, name, convo
- needs goal properties
 - npc pos, loctype, name, convo

*/

#[derive(Clone, Debug, PartialEq)]
pub enum TaskType {
    Plot,
    RetrieveItem,
    PassMessage,
    PassItem,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Task {
    BoardItemWanted {
        receiver_loc: (i16, i16),
        receiver_loc_name: String,
        receiver_entity_name: String,
        receiver_convo: Convo,
        task_items: (Items, u8),
        // task_items: HashMap<Items, u8>,
        note_entries: Vec<(bool, String)>,
        complete: bool,
        reward: Item,
    },
    RetrieveItem {
        start_loc: (i16, i16),
        start_loc_name: String,
        start_entity_name: String,
        start_convo: Convo,
        goal_loc: (i16, i16),
        goal_loc_name: String,
        goal_entity_name: String,
        goal_convo: Convo,
        task_items: Option<Vec<(bool, Item)>>,
        note_entries: Vec<(bool, String)>,
        reward: Item,
    },
}

impl Task {
    pub fn new_board_item_wanted_task(receiver_loc: (i16, i16), receiver_loc_name: String) -> Self {
        let receiver_entity_name = "Eric".to_string();
        let reward = Item::new_health_potion(0, 0);
        let task_items = (Items::Apple, 1);
        let null_comms = vec!["Hey there."];

        let mut stages = HashMap::new();
        stages.insert(
            "0".to_string(),
            Stage {
                text: "Thanks for that! Ill let the guild know you grabbed this for me."
                    .to_string(),
                opts: vec![
                    ConOpt {
                        text: "No problem.".to_string(),
                        next: "e".to_string(),
                    },
                    ConOpt {
                        text: "Sure thing.".to_string(),
                        next: "e".to_string(),
                    },
                ],
            },
        );
        let receiver_convo = Convo {
            id: "Item Wanted: Goal".to_string(),
            stages,
        };

        // let convo_path = format!("src/npcs/{}/convos_city.json", "task");
        // let data1 = fs::read_to_string(convo_path);
        // print!("{:?}", data1);
        // let convos: Vec<Convo> = match data1 {
        //     Ok(content) => serde_json::from_str(&content).unwrap(),
        //     Err(e) => {
        //         log::info!("{:?}", e);
        //         Vec::new()
        //     }
        // };

        let note_entries = vec![
            (
                false,
                format!(
                    r#"
{} is looking for {} {}.

They live in {}, and can be found there to deliver the item.
            "#,
                    receiver_entity_name, task_items.1, task_items.0, receiver_loc_name
                ),
            ),
            (
                false,
                format!(
                    r#"
You have delivered the {} to {}.

You can now report to Guild Head for payment.
            "#,
                    task_items.1, receiver_entity_name,
                ),
            ),
        ];

        Self::BoardItemWanted {
            receiver_loc,
            receiver_loc_name,
            receiver_entity_name,
            receiver_convo,
            task_items,
            note_entries,
            complete: false,
            reward,
        }
    }

    pub fn board_post(&mut self) -> String {
        match self {
            Task::BoardItemWanted {
                receiver_loc_name,
                receiver_entity_name,
                task_items,
                reward,
                ..
            } => {
                format!(
                    r#"
---- ---- nl
Item Retrieval nl
nl
{} in {} needs {}. nl
nl
{} is looking for {} {}, and is looking to provide {} gold in payment. nl
____ nl
                "#,
                    receiver_entity_name,
                    receiver_loc_name,
                    task_items.0,
                    receiver_entity_name,
                    task_items.1,
                    task_items.0,
                    reward.get_properties()["value"]
                )
            }
            _ => "oops".to_string(),
        }
    }

    pub fn note_entry(&self) -> String {
        let mut entries = match self {
            Task::BoardItemWanted { note_entries, .. } => note_entries.clone(),
            _ => vec![(false, "oops".to_string())],
        };
        entries.retain(|n| !n.0);
        entries[0].1.clone()
    }

    pub fn complete_task(&mut self) {
        match self {
            Task::BoardItemWanted { complete, .. } => *complete = true,
            _ => {}
        }
    }

    pub fn is_complete(&mut self) -> bool {
        match self {
            Task::BoardItemWanted { complete, .. } => *complete,
            _ => false,
        }
    }

    pub fn reward(&self) -> Item {
        match self {
            Task::BoardItemWanted { reward, .. } => reward.clone(),
            _ => Item::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tasks {
    pub tasks: Vec<Task>,
    pub locals: Vec<((i16, i16), String)>,
    pub task_locations: Vec<((i16, i16), String)>,
    pub active_tasks: Vec<Task>,
    pub board_tasks: Vec<Task>,
    pub active_board_task: Option<Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            locals: Vec::new(),
            task_locations: Vec::new(),
            active_tasks: Vec::new(),
            board_tasks: Vec::new(),
            active_board_task: None,
        }
    }

    pub fn set_board_task(&mut self, task: Task) {
        let idx = self.board_tasks.iter().position(|x| *x == task).unwrap();
        self.board_tasks.remove(idx);
        self.active_board_task = Some(task);
    }

    pub fn new_board_task(&mut self) {
        let mut rng = rand::thread_rng();
        let ttype = [
            // TaskType::Plot,
            TaskType::RetrieveItem,
            // TaskType::PassMessage,
            // TaskType::PassItem,
        ]
        .choose(&mut rng)
        .unwrap_or(&TaskType::RetrieveItem);

        match ttype {
            TaskType::Plot => {}
            TaskType::RetrieveItem => {
                let start_loc = self.locals.pop().unwrap();
                self.board_tasks
                    .push(Task::new_board_item_wanted_task(start_loc.0, start_loc.1));
            }
            TaskType::PassMessage => {}
            TaskType::PassItem => {}
        }
    }
}
