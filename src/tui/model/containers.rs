use super::super::common::*;
use aws_sdk_ecs::types::Container;
use ratatui::{
    text::Line,
    widgets::{ListItem, ListState},
};

#[derive(Debug)]
pub struct ContainerItem {
    pub container: Container,
    pub status: bool,
}

impl ContainerItem {
    fn new(container: Container) -> Self {
        Self {
            container,
            status: false,
        }
    }
}

#[derive(Debug)]
pub struct ContainerItems {
    pub items: Vec<ContainerItem>,
    pub state: ListState,
}

impl ContainerItems {
    pub fn empty() -> Self {
        Self::from(Vec::new().as_slice())
    }
}

impl From<&ContainerItem> for ListItem<'_> {
    fn from(value: &ContainerItem) -> Self {
        let pending_marker = if let Some(status) = value.container.last_status() {
            if status != "RUNNING" { " ~" } else { "" }
        } else {
            ""
        };
        let line = Line::from(
            value
                .container
                .name()
                .map(|n| format!("{}{}", n, pending_marker))
                .unwrap_or(UNKNOWN_VALUE.to_string()),
        );
        ListItem::new(line)
    }
}

impl From<&[Container]> for ContainerItems {
    fn from(containers: &[Container]) -> Self {
        let mut items: Vec<ContainerItem> = containers
            .iter()
            .map(|container| ContainerItem::new(container.clone()))
            .collect();

        items.sort_by(|a, b| {
            a.container
                .name()
                .unwrap_or_default()
                .cmp(b.container.name().unwrap_or_default())
        });

        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}
