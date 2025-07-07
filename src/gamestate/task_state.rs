// task_state.rs

use crate::tasks::Task;

use super::GameState;

impl GameState {
    pub fn pick_board_task(&mut self, task: Task) {
        self.tasks.set_board_task(task.clone());
        self.notebook
            .enter_tasks("Guild Posting".to_string(), task.note_entry());
        self.settles.set_task_content(task.clone());
    }
}
