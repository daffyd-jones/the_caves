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
pub struct Task {
    pub ttype: TaskType,
    pub start_loc: (i16, i16),
    pub start_loc_name: String,
    pub start_entity_name: String,
    pub goal_loc: (i16, i16),
    pub goal_loc_name: String,
    pub goal_entity_name: String,
    pub reward: Item,
    pub task_items: Option<Vec<(bool, Item)>>,
    pub start_convo: Convo,
    pub goal_convo: Convo,
    pub final_convo: Option<Convo>,
    pub note_entries: Vec<(bool, String)>,
    pub stat_triggers: Vec<ToggleState>,
}

impl Task {
    pub fn new_retrieve_task(start_loc: (i16, i16), start_loc_name: String) -> Self {
        let start_entity_name = "Daniel".to_string();
        let goal_entity_name = "Eric".to_string();
        let reward = Item::new_health_potion(0, 0);
        let task_items = Some(vec![(false, Item::new_apple(0, 0))]);
        let mut stages = HashMap::new();
        stages.insert(
            "0".to_string(),
            Stage {
                text: "This is npc dialogue.".to_string(),
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
        let start_convo = Convo {
            id: "Retrieve Item".to_string(),
            stages,
        };

        let mut stages = HashMap::new();
        stages.insert(
            "0".to_string(),
            Stage {
                text: "This is npc dialogue.".to_string(),
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
            id: "Retrieve Item".to_string(),
            stages,
        };

        let mut stages = HashMap::new();
        stages.insert(
            "0".to_string(),
            Stage {
                text: "This is npc dialogue.".to_string(),
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
        let final_convo = Some(Convo {
            id: "Retrieve Item".to_string(),
            stages,
        });

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
                start_entity_name,
                task_items.clone().unwrap().len(),
                task_items.clone().unwrap()[0].1.sname,
                start_loc_name,
                reward.sname
            ),
        )];
        let stat_triggers = Vec::new();

        Self {
            ttype: TaskType::RetrieveItem,
            start_loc,
            start_loc_name,
            start_entity_name,
            goal_loc: (0, 0),
            goal_loc_name: "".to_string(),
            goal_entity_name,
            reward,
            task_items,
            start_convo,
            goal_convo,
            final_convo,
            note_entries,
            stat_triggers,
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
                    .push(Task::new_retrieve_task(start_loc.0, start_loc.1));
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
