use super::super::common::*;
use aws_sdk_ecs::types::Task;
use ratatui::{
    text::Line,
    widgets::{ListItem, ListState},
};

#[derive(Debug)]
pub struct TaskItems {
    pub items: Vec<TaskItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct TaskItem {
    pub task: Task,
    pub status: bool,
}

impl TaskItem {
    fn new(task: Task) -> Self {
        Self {
            task,
            status: false,
        }
    }
}

impl From<&TaskItem> for ListItem<'_> {
    fn from(value: &TaskItem) -> Self {
        let pending_marker = if let Some(status) = value.task.last_status() {
            if status != "RUNNING" { " ~" } else { "" }
        } else {
            ""
        };

        let line = Line::from(
            value
                .task
                .task_arn()
                .and_then(|arn| arn.split("/").last())
                .map(|id| format!("{}{}", id, pending_marker))
                .unwrap_or(UNKNOWN_VALUE.to_string()),
        );

        ListItem::new(line)
    }
}

impl From<&Vec<Task>> for TaskItems {
    fn from(tasks: &Vec<Task>) -> Self {
        let items = tasks
            .iter()
            .map(|task| TaskItem::new(task.clone()))
            .collect();
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}
