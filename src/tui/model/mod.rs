mod containers;
mod services;
mod tasks;

pub use containers::*;
pub use services::*;
pub use tasks::*;

use super::common::*;
use crate::config::ClusterConfig;
use crate::domain::*;
use aws_sdk_ecs::types::Task;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum UserMessage {
    Info(String, Instant),
    Error(String, Instant),
}

#[allow(dead_code)]
impl UserMessage {
    pub(super) fn info(message: &str) -> Self {
        UserMessage::Info(message.to_string(), Instant::now())
    }
    pub(super) fn error(message: &str) -> Self {
        UserMessage::Error(message.to_string(), Instant::now())
    }
}

#[allow(unused)]
pub struct Model {
    pub profile_name: String,
    pub clusters: Vec<ClusterConfig>,
    pub active_pane: Pane,
    pub last_active_pane: Option<Pane>,
    pub running_state: RunningState,
    pub task_results_cache: HashMap<ServiceDetails, Vec<Task>>,
    pub num_fetches_in_flight: usize,
    pub num_errors: usize,
    pub service_items: ServiceItems,
    pub task_items: Option<TaskItems>,
    pub container_items: ContainerItems,
    pub user_message: Option<UserMessage>,
    pub terminal_dimensions: TerminalDimensions,
    pub terminal_too_small: bool,
    pub render_counter: u64,
    pub event_counter: u64,
    pub num_services_marked_for_refresh: usize,
    pub auto_refresh: bool,
    pub debug: bool,
    pub redact_mode: bool,
}

impl Model {
    pub fn new(
        profile_name: String,
        clusters: Vec<ClusterConfig>,
        terminal_dimensions: TerminalDimensions,
        debug: bool,
        redact_mode: bool,
    ) -> Self {
        let terminal_too_small = terminal_dimensions.width < MIN_TERMINAL_WIDTH
            || terminal_dimensions.height < MIN_TERMINAL_HEIGHT;

        Self {
            profile_name,
            clusters,
            active_pane: Pane::ServicesList,
            last_active_pane: None,
            running_state: RunningState::Running,
            task_results_cache: HashMap::new(),
            num_fetches_in_flight: 0,
            num_errors: 0,
            service_items: ServiceItems::empty(),
            task_items: None,
            container_items: ContainerItems::empty(),
            user_message: None,
            terminal_dimensions,
            terminal_too_small,
            render_counter: 0,
            event_counter: 0,
            num_services_marked_for_refresh: 0,
            auto_refresh: false,
            debug,
            redact_mode,
        }
    }

    pub(super) fn go_back_or_quit(&mut self) {
        let active_pane = Some(self.active_pane);
        match self.active_pane {
            Pane::ServicesList => self.running_state = RunningState::Done,
            Pane::ServiceDetails => self.active_pane = Pane::ServicesList,
            Pane::TasksList => self.active_pane = Pane::ServicesList,
            Pane::TaskDetails => self.active_pane = Pane::TasksList,
            Pane::ContainersList => self.active_pane = Pane::TasksList,
            Pane::ContainerDetails => self.active_pane = Pane::ContainersList,
            Pane::Help => self.active_pane = self.last_active_pane.unwrap_or(Pane::ServicesList),
        }

        self.last_active_pane = active_pane;
    }

    pub(super) fn select_next_list_item(&mut self) {
        match self.active_pane {
            Pane::ServicesList => self.service_items.state.select_next(),
            Pane::ServiceDetails => {}
            Pane::TasksList => {
                if let Some(i) = &mut self.task_items {
                    i.state.select_next()
                }
            }
            Pane::TaskDetails => {}
            Pane::ContainersList => self.container_items.state.select_next(),
            Pane::ContainerDetails => {}
            Pane::Help => {}
        }
    }

    pub(super) fn select_previous_list_item(&mut self) {
        match self.active_pane {
            Pane::ServicesList => self.service_items.state.select_previous(),
            Pane::ServiceDetails => {}
            Pane::TasksList => {
                if let Some(i) = &mut self.task_items {
                    i.state.select_previous()
                }
            }
            Pane::TaskDetails => {}
            Pane::ContainersList => self.container_items.state.select_previous(),
            Pane::ContainerDetails => {}
            Pane::Help => {}
        }
    }

    pub(super) fn select_first_list_item(&mut self) {
        match self.active_pane {
            Pane::ServicesList => self.service_items.state.select_first(),
            Pane::TasksList => {
                if let Some(i) = &mut self.task_items {
                    i.state.select_first()
                }
            }
            Pane::ContainersList => self.container_items.state.select_first(),
            _ => {}
        }
    }
    pub(super) fn select_last_list_item(&mut self) {
        match self.active_pane {
            Pane::ServicesList => self.service_items.state.select_last(),
            Pane::TasksList => {
                if let Some(i) = &mut self.task_items {
                    i.state.select_last()
                }
            }
            Pane::ContainersList => self.container_items.state.select_last(),
            _ => {}
        }
    }

    pub fn get_selected_service(&self) -> Option<(&ServiceResult, usize)> {
        let service_index = self.service_items.state.selected()?;

        self.service_items
            .items
            .get(service_index)
            .map(|si| (&si.service, service_index))
    }

    pub fn get_selected_task(&self) -> Option<&Task> {
        match &self.task_items {
            Some(i) => match i.state.selected() {
                Some(task_index) => match i.items.get(task_index) {
                    Some(ti) => Some(&ti.task),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}
