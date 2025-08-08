use super::command::Command;
use super::common::*;
use super::message::Message;
use super::model::*;
use std::time::Instant;

pub fn update(model: &mut Model, msg: Message) -> Vec<Command> {
    let service_index_before_update = model.service_items.state.selected();
    let task_index_before_update = match &model.task_items {
        Some(i) => i.state.selected(),
        None => None,
    };
    let mut data_refresh = false;

    let mut cmds = Vec::new();

    match msg {
        Message::GoToNextListItem => model.select_next_list_item(),
        Message::GoToPreviousListItem => model.select_previous_list_item(),
        Message::GoToFirstListItem => model.select_first_list_item(),
        Message::GoToLastListItem => model.select_last_list_item(),
        Message::TerminalResize(width, height) => {
            model.terminal_dimensions = TerminalDimensions { width, height };
            model.terminal_too_small =
                !(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT);
        }
        Message::ClearUserMsg => {
            let now = Instant::now();
            let reset_message = match &model.user_message {
                Some(message) => match message {
                    UserMessage::Info(_, instant) => {
                        now.saturating_duration_since(instant.to_owned()).as_secs()
                            > CLEAR_USER_MESSAGE_LOOP_INTERVAL_SECS
                    }
                    UserMessage::Error(_, instant) => {
                        now.saturating_duration_since(instant.to_owned()).as_secs()
                            > CLEAR_USER_MESSAGE_LOOP_INTERVAL_SECS
                    }
                },
                None => false,
            };

            if reset_message {
                model.user_message = None;
            }
        }
        Message::GoToPane(pane) => {
            model.last_active_pane = Some(model.active_pane);
            model.active_pane = pane;
        }
        Message::TasksFetched((service_details, tasks, refresh)) => {
            model.task_results_cache.insert(service_details, tasks);
            data_refresh = refresh;
        }
        Message::ServicesFetched(service_results) => {
            model.service_items.append(&service_results);
            for service_result in &service_results {
                if let Ok(service_details) = service_result {
                    cmds.push(Command::GetTasks((service_details.clone(), false)));
                } else {
                    model.num_errors += 1;
                }
            }
        }
        Message::ServiceDetailsRefreshed((result, previous_service_details, index)) => {
            if model.service_items.items.len() > index {
                let service_item = ServiceItem::new(result.clone());
                let marked_for_refresh = model
                    .service_items
                    .items
                    .get(index)
                    .is_some_and(|i| i.marked_for_refresh);
                model.service_items.items[index] = service_item;
                if result.is_ok() {
                    model.service_items.items[index].marked_for_refresh = marked_for_refresh;
                    model.task_results_cache.remove(&previous_service_details);
                    data_refresh = true;
                } else if marked_for_refresh {
                    model.service_items.items[index].marked_for_refresh = false;
                    model.num_services_marked_for_refresh -= 1;
                }
            }
        }
        Message::RefreshResultsForCurrentItem => match model.active_pane {
            Pane::ServicesList | Pane::ServiceDetails => {
                if let Some((Ok(service_details), index)) = model.get_selected_service() {
                    cmds.push(Command::RefreshService((service_details.clone(), index)));
                }
            }
            Pane::TasksList | Pane::TaskDetails | Pane::ContainersList | Pane::ContainerDetails => {
                if let Some((Ok(service_details), _)) = model.get_selected_service() {
                    cmds.push(Command::GetTasks((service_details.clone(), true)));
                    model.task_results_cache.remove(&service_details.clone());
                    model.task_items = None;
                }
            }
            Pane::Help => {}
        },
        Message::RefreshResultsForMarkedServices => match model.active_pane {
            Pane::Help => {}
            _ => {
                if model.num_services_marked_for_refresh == 0 {
                    for (index, service_result) in model.service_items.items.iter_mut().enumerate()
                    {
                        if let Ok(service_details) = &service_result.service {
                            cmds.push(Command::RefreshService((service_details.clone(), index)));
                            model.task_results_cache.remove(service_details);
                        }
                    }
                } else {
                    for (index, service_result) in model.service_items.items.iter_mut().enumerate()
                    {
                        if service_result.marked_for_refresh
                            && let Ok(service_details) = &service_result.service {
                                cmds.push(Command::RefreshService((
                                    service_details.clone(),
                                    index,
                                )));
                                model.task_results_cache.remove(service_details);
                            }
                    }
                }
            }
        },
        Message::ToggleServiceRefresh => {
            if let Some(index) = model.service_items.state.selected()
                && let Some(service_item) = model.service_items.items.get_mut(index) {
                    if service_item.service.is_ok() {
                        if service_item.marked_for_refresh {
                            service_item.marked_for_refresh = false;
                            model.num_services_marked_for_refresh -= 1;
                        } else {
                            service_item.marked_for_refresh = true;
                            model.num_services_marked_for_refresh += 1;
                        }
                    } else {
                        model.user_message = Some(UserMessage::error(
                            "error results cannot be marked for refresh",
                        ));
                    }
                }
        }
        Message::ToggleAutoRefresh => model.auto_refresh = !model.auto_refresh,
        Message::GoBackOrQuit => model.go_back_or_quit(),
        Message::QuitImmediately => model.running_state = RunningState::Done,
    }

    let refresh_tasks_and_containers = data_refresh
        || model.task_items.is_none()
        || service_index_before_update != model.service_items.state.selected();

    if refresh_tasks_and_containers {
        if let Some((service_result, _)) = model.get_selected_service() {
            match service_result {
                Ok(service) => {
                    match model.task_results_cache.get(service) {
                        Some(tr) => {
                            let task_items = TaskItems::from(tr);
                            model.task_items = Some(task_items);
                        }
                        None => {
                            cmds.push(Command::GetTasks((service.clone(), false)));
                            model.task_items = None;
                        }
                    }

                    if let Some(selected_task) = model.get_selected_task() {
                        let container_items = ContainerItems::from(selected_task.containers());
                        model.container_items = container_items;
                    }
                }
                Err(_) => {
                    model.task_items = None;
                    model.container_items = ContainerItems::empty();
                }
            }
        }
    } else if let Some(i) = &model.task_items
        && task_index_before_update != i.state.selected()
            && let Some(selected_task) = model.get_selected_task() {
                let container_items = ContainerItems::from(selected_task.containers());
                model.container_items = container_items;
            }

    cmds
}
