// task_state.rs

use crate::{
    enums::{EnvInter, Location, TaskEnv},
    tasks::Task,
};

use super::GameState;

impl GameState {
    pub fn pick_board_task(&mut self, task: Task) {
        self.tasks.set_board_task(task.clone());
        self.notebook
            .enter_tasks("Guild Posting".to_string(), task.note_entry());
        if !self.check_current_location(task.clone()) {
            self.settles.set_task_content(task.clone());
        }
    }

    pub fn check_current_location(&mut self, task: Task) -> bool {
        if let Location::Settlement(ref mut settle) = self.location.clone() {
            match task {
                Task::BoardItemWanted { receiver_loc, .. } if receiver_loc == settle.pos => {
                    settle.add_task_env(EnvInter::TaskEnv(TaskEnv::BoardGoalEntity));
                    self.location = Location::Settlement(settle.clone());
                    return true;
                }
                _ => return false,
            }
        }
        false
    }
}
