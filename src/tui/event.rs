use super::common::*;
use super::message::Message;
use super::model::*;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub fn get_event_handling_msg(model: &Model, event: Event) -> Option<Message> {
    match event {
        Event::Key(key_event) => match model.terminal_too_small {
            true => match key_event.kind {
                KeyEventKind::Press => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                    _ => None,
                },
                _ => None,
            },
            false => match key_event.kind {
                KeyEventKind::Press => match model.active_pane {
                    Pane::ServicesList => match key_event.code {
                        KeyCode::Char('2') => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('3') => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Char('j') | KeyCode::Down => Some(Message::GoToNextListItem),
                        KeyCode::Char('k') | KeyCode::Up => Some(Message::GoToPreviousListItem),
                        KeyCode::Char('g') => Some(Message::GoToFirstListItem),
                        KeyCode::Char('G') => Some(Message::GoToLastListItem),
                        KeyCode::Right => Some(Message::GoToPane(Pane::ServiceDetails)),
                        KeyCode::Char('J') => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('L') | KeyCode::Char('H') => {
                            Some(Message::GoToPane(Pane::ServiceDetails))
                        }
                        KeyCode::Char('K') => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Tab => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::BackTab => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Char('?') => Some(Message::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                        KeyCode::Char('m') => Some(Message::ToggleServiceRefresh),
                        KeyCode::Char('R') => Some(Message::ToggleAutoRefresh),
                        KeyCode::Char('r') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::RefreshResultsForMarkedServices)
                            } else {
                                Some(Message::RefreshResultsForCurrentItem)
                            }
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Pane::ServiceDetails => match key_event.code {
                        KeyCode::Char('2') => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('3') => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Left => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('J') => Some(Message::GoToPane(Pane::TaskDetails)),
                        KeyCode::Char('L') | KeyCode::Char('H') => {
                            Some(Message::GoToPane(Pane::ServicesList))
                        }
                        KeyCode::Char('K') => Some(Message::GoToPane(Pane::ContainerDetails)),
                        KeyCode::Tab => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::BackTab => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Char('?') => Some(Message::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                        KeyCode::Char('r') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::RefreshResultsForMarkedServices)
                            } else {
                                Some(Message::RefreshResultsForCurrentItem)
                            }
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Pane::TasksList => match key_event.code {
                        KeyCode::Char('1') => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('3') => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Char('4') => Some(Message::GoToPane(Pane::ContainerDetails)),
                        KeyCode::Char('j') | KeyCode::Down => Some(Message::GoToNextListItem),
                        KeyCode::Char('k') | KeyCode::Up => Some(Message::GoToPreviousListItem),
                        KeyCode::Char('g') => Some(Message::GoToFirstListItem),
                        KeyCode::Char('G') => Some(Message::GoToLastListItem),
                        KeyCode::Right => Some(Message::GoToPane(Pane::TaskDetails)),
                        KeyCode::Char('J') => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Char('L') | KeyCode::Char('H') => {
                            Some(Message::GoToPane(Pane::TaskDetails))
                        }
                        KeyCode::Char('K') => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Tab => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::BackTab => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('?') => Some(Message::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                        KeyCode::Char('r') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::RefreshResultsForMarkedServices)
                            } else {
                                Some(Message::RefreshResultsForCurrentItem)
                            }
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Pane::TaskDetails => match key_event.code {
                        KeyCode::Char('1') => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('3') => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Char('4') => Some(Message::GoToPane(Pane::ContainerDetails)),
                        KeyCode::Left => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('J') => Some(Message::GoToPane(Pane::ContainerDetails)),
                        KeyCode::Char('L') | KeyCode::Char('H') => {
                            Some(Message::GoToPane(Pane::TasksList))
                        }
                        KeyCode::Char('K') => Some(Message::GoToPane(Pane::ServiceDetails)),
                        KeyCode::Tab => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::BackTab => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('?') => Some(Message::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                        KeyCode::Char('r') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::RefreshResultsForMarkedServices)
                            } else {
                                Some(Message::RefreshResultsForCurrentItem)
                            }
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Pane::ContainersList => match key_event.code {
                        KeyCode::Char('1') => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('2') => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('4') => Some(Message::GoToPane(Pane::ContainerDetails)),
                        KeyCode::Char('j') | KeyCode::Down => Some(Message::GoToNextListItem),
                        KeyCode::Char('k') | KeyCode::Up => Some(Message::GoToPreviousListItem),
                        KeyCode::Char('g') => Some(Message::GoToFirstListItem),
                        KeyCode::Char('G') => Some(Message::GoToLastListItem),
                        KeyCode::Right => Some(Message::GoToPane(Pane::ContainerDetails)),
                        KeyCode::Char('J') => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('L') | KeyCode::Char('H') => {
                            Some(Message::GoToPane(Pane::ContainerDetails))
                        }
                        KeyCode::Char('K') => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Tab => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::BackTab => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('?') => Some(Message::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                        KeyCode::Char('r') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::RefreshResultsForMarkedServices)
                            } else {
                                Some(Message::RefreshResultsForCurrentItem)
                            }
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Pane::ContainerDetails => match key_event.code {
                        KeyCode::Char('1') => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::Char('2') => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('3') => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Left => Some(Message::GoToPane(Pane::ContainersList)),
                        KeyCode::Char('J') => Some(Message::GoToPane(Pane::ServiceDetails)),
                        KeyCode::Char('L') | KeyCode::Char('H') => {
                            Some(Message::GoToPane(Pane::ContainersList))
                        }
                        KeyCode::Char('K') => Some(Message::GoToPane(Pane::TaskDetails)),
                        KeyCode::Tab => Some(Message::GoToPane(Pane::ServicesList)),
                        KeyCode::BackTab => Some(Message::GoToPane(Pane::TasksList)),
                        KeyCode::Char('?') => Some(Message::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                        KeyCode::Char('r') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::RefreshResultsForMarkedServices)
                            } else {
                                Some(Message::RefreshResultsForCurrentItem)
                            }
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Pane::Help => match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
                            Some(Message::GoBackOrQuit)
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                },
                _ => None,
            },
        },
        Event::Resize(w, h) => Some(Message::TerminalResize(w, h)),
        _ => None,
    }
}
