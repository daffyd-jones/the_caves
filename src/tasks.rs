// tasks.rs

use crate::enums::ToggleState;
use crate::item::Item;
use crate::npc::Convo;

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

enum TaskType {
    Plot,
    RetrieveItem,
    PassMessage,
    PassItem,
}

struct Task {
    ttype: TaskType,
    start_pos: (i64, i64),
    goal_pos: (i64, i64),
    reward: Item,
    task_item: Item,
    start_name: String,
    goal_name: String,
    start_convo: Convo,
    goal_convo: Convo,
    final_convo: Option<Convo>,
    stat_triggers: Vec<ToggleState>,
}

struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    // pub fn new_retrieve_task() -> Self {
    //     Self {

    //     }
    // }
}
