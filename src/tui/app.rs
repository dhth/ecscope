use super::command::Command;
use super::common::*;
use super::event::get_event_handling_msg;
use super::handle::handle_command;
use super::message::Message;
use super::model::*;
use super::update::update;
use super::view::view;
use crate::config::{ClusterConfig, ConfigSource};
use aws_sdk_ecs::Client as ECSClient;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

const EVENT_POLL_DURATION_MS: u64 = 16;
pub const REFRESH_RESULTS_INTERVAL_SECS: u64 = 10;

pub async fn run_tui(
    profile_name: String,
    clients_map: HashMap<ConfigSource, ECSClient>,
    clusters: Vec<ClusterConfig>,
) -> anyhow::Result<()> {
    let mut tui = AppTui::new(profile_name, clusters)?;
    tui.run(clients_map).await?;

    Ok(())
}

struct AppTui {
    pub(super) terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub(super) event_tx: Sender<Message>,
    pub(super) event_rx: Receiver<Message>,
    pub(super) model: Model,
    pub(super) initial_commands: Vec<Command>,
}

impl AppTui {
    pub fn new(profile_name: String, clusters: Vec<ClusterConfig>) -> anyhow::Result<Self> {
        let terminal = ratatui::try_init()?;
        let (event_tx, event_rx) = mpsc::channel(10);
        let mut initial_commands = Vec::new();
        for cluster in &clusters {
            initial_commands.push(Command::GetServices(cluster.clone()));
        }

        let (width, height) = ratatui::crossterm::terminal::size()?;

        let terminal_dimensions = TerminalDimensions { width, height };

        let debug = std::env::var("ECSCOPE_DEBUG").unwrap_or_default().trim() == "1";
        let redact_mode = std::env::var("ECSCOPE_REDACT").unwrap_or("0".to_string()) == "1";

        let model = Model::new(
            profile_name,
            clusters,
            terminal_dimensions,
            debug,
            redact_mode,
        );

        Ok(Self {
            terminal,
            event_tx,
            event_rx,
            model,
            initial_commands,
        })
    }

    pub async fn run(
        &mut self,
        clients_map: HashMap<ConfigSource, ECSClient>,
    ) -> anyhow::Result<()> {
        let message_clear_duration = Duration::from_secs(CLEAR_USER_MESSAGE_LOOP_INTERVAL_SECS);
        let refresh_results_duration = Duration::from_secs(REFRESH_RESULTS_INTERVAL_SECS);
        let mut message_clear_interval = tokio::time::interval(message_clear_duration);
        let mut refresh_results_interval = tokio::time::interval(refresh_results_duration);
        let _ = self.terminal.clear();
        let clients_map = Arc::new(clients_map);

        for cmd in &self.initial_commands {
            handle_command(Arc::clone(&clients_map), cmd.clone(), self.event_tx.clone()).await;
        }

        // first render
        self.model.render_counter += 1;
        self.terminal.draw(|f| view(&mut self.model, f))?;

        loop {
            tokio::select! {
                _instant = message_clear_interval.tick() => {
                    if self.model.user_message.is_some() {
                        _ = self.event_tx.try_send(Message::ClearUserMsg);
                    }
                }

                _instant = refresh_results_interval.tick() => {
                    if self.model.auto_refresh {
                        _ = self.event_tx.try_send(Message::RefreshResultsForMarkedServices);
                    }
                }

                Some(message) = self.event_rx.recv() => {
                    let cmds = update(&mut self.model, message);

                    if self.model.running_state == RunningState::Done {
                        self.exit()?;
                        return Ok(());
                    }

                        self.model.render_counter += 1;
                        self.terminal.draw(|f| view(&mut self.model, f))?;

                    for cmd in cmds {
                        handle_command(Arc::clone(&clients_map), cmd.clone(), self.event_tx.clone()).await;
                    }
                }

                Ok(ready) = tokio::task::spawn_blocking(|| ratatui::crossterm::event::poll(Duration::from_millis(EVENT_POLL_DURATION_MS))) => {
                    match ready {
                        Ok(true) => {
                            let event = ratatui::crossterm::event::read()?;
                            self.model.event_counter += 1;
                            if let Some(handling_msg) = get_event_handling_msg(&self.model, event) {
                                self.event_tx.try_send(handling_msg)?;
                            }
                        }
                        Ok(false) => continue,
                        Err(e) => {
                                return Err(anyhow::anyhow!(e));
                        }
                    }
                }
            }
        }
    }

    fn exit(&mut self) -> Result<(), IOError> {
        ratatui::try_restore()
    }
}
