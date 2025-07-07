// tasks.rs

use std::collections::HashMap;
use std::fs;

use rand::seq::SliceRandom;

use crate::enums::{Location, ToggleState};
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
    BoardRetrieveItem {
        goal_loc: (i16, i16),
        goal_loc_name: String,
        goal_entity_name: String,
        goal_convo: Convo,
        task_items: Option<Vec<(bool, Item)>>,
        note_entries: Vec<(bool, String)>,
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
    pub fn new_board_retrieve_task(goal_loc: (i16, i16), goal_loc_name: String) -> Self {
        let goal_entity_name = "Eric".to_string();
        let reward = Item::new_health_potion(0, 0);
        let task_items = Some(vec![(false, Item::new_apple(0, 0))]);

        let mut stages = HashMap::new();
        stages.insert(
            "0".to_string(),
            Stage {
                text: "This is npc dialogue when completing goal.".to_string(),
                opts: vec![
                    ConOpt {
                        text: "Thank's Ill look for it. Good luck!".to_string(),
                        next: "e".to_string(),
                    },
                    ConOpt {
                        text: "What did it look like?".to_string(),
                        next: "desc".to_string(),
                    },
                ],
            },
        );
        let goal_convo = Convo {
            id: "Retrieve Item: Goal".to_string(),
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

        let note_entries = vec![(
            false,
            format!(
                r#"
{} is looking for {} {}.

They live in {}, and can be found there to deliver the item.

They are willing to give {} as a reward.
            "#,
                goal_entity_name,
                task_items.clone().unwrap().len(),
                task_items.clone().unwrap()[0].1.sname,
                goal_loc_name,
                reward.sname
            ),
        )];

        Self::BoardRetrieveItem {
            goal_loc,
            goal_loc_name,
            goal_entity_name,
            goal_convo,
            task_items,
            note_entries,
            reward,
        }
    }

    pub fn board_post(&mut self) -> String {
        match self {
            Task::BoardRetrieveItem {
                goal_loc_name,
                goal_entity_name,
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
                    goal_entity_name,
                    goal_loc_name,
                    task_items.clone().unwrap()[0].1.sname,
                    goal_entity_name,
                    task_items.clone().unwrap().len(),
                    task_items.clone().unwrap()[0].1.sname,
                    reward.get_properties()["value"]
                )
            }
            _ => "oops".to_string(),
        }
    }

    pub fn note_entry(&self) -> String {
        let mut entries = match self {
            Task::BoardRetrieveItem { note_entries, .. } => note_entries.clone(),
            _ => vec![(false, "oops".to_string())],
        };
        entries.retain(|n| !n.0);
        entries[0].1.clone()
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
            TaskType::Plot => {
                // self.board_task.push(Task::new_retrieve_task(
                //     start_loc,
                //     start_loc_name,
                //     start_entity_name,
                //     goal_loc,
                //     goal_loc_name,
                //     goal_entity_name,
                //     reward,
                //     task_item,
                //     start_convo,
                //     goal_convo,
                //     final_convo,
                //     note_entries,
                //     stat_triggers,
                // ));
            }
            TaskType::RetrieveItem => {
                let start_loc = self.locals.pop().unwrap();
                self.board_tasks
                    .push(Task::new_board_retrieve_task(start_loc.0, start_loc.1));
            }
            TaskType::PassMessage => {
                // self.board_task.push(Task::new_retrieve_task(
                //     start_loc,
                //     start_loc_name,
                //     start_entity_name,
                //     goal_loc,
                //     goal_loc_name,
                //     goal_entity_name,
                //     reward,
                //     task_item,
                //     start_convo,
                //     goal_convo,
                //     final_convo,
                //     note_entries,
                //     stat_triggers,
                // ));
            }
            TaskType::PassItem => {
                // self.board_task.push(Task::new_retrieve_task(
                //     start_loc,
                //     start_loc_name,
                //     start_entity_name,
                //     goal_loc,
                //     goal_loc_name,
                //     goal_entity_name,
                //     reward,
                //     task_item,
                //     start_convo,
                //     goal_convo,
                //     final_convo,
                //     note_entries,
                //     stat_triggers,
                // ));
            }
        }
    }
}
