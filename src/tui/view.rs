use super::common::*;
use super::model::{Model, UserMessage};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListDirection, ListItem, Padding, Paragraph, Wrap},
};

const HELP_CONTENTS: &str = include_str!("static/help.txt");
const REDACTED: &str = "<REDACTED>";

pub fn view(model: &mut Model, frame: &mut Frame) {
    if model.terminal_too_small {
        render_terminal_too_small_view(&model.terminal_dimensions, frame);
        return;
    }

    match model.active_pane {
        Pane::Help => render_help_view(model, frame),
        _ => render_list_view(model, frame),
    }
}

fn render_terminal_too_small_view(dimensions: &TerminalDimensions, frame: &mut Frame) {
    let message = format!(
        r#"
Terminal size too small:
  Width = {} Height = {}

Minimum dimensions needed:
  Width = {} Height = {}

Press (q/<ctrl+c>/<esc> to exit)
"#,
        dimensions.width, dimensions.height, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT
    );

    let p = Paragraph::new(message)
        .block(Block::bordered())
        .style(Style::new().fg(PRIMARY_COLOR))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    frame.render_widget(p, frame.area());
}

fn render_help_view(model: &mut Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Min(21), Constraint::Length(1)])
        .split(frame.area());

    let lines: Vec<Line<'_>> = HELP_CONTENTS.lines().map(Line::from).collect();

    let p = Paragraph::new(lines)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(MESSAGE_COLOR))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(MESSAGE_COLOR)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(" help ")
                .padding(Padding::new(2, 0, 1, 0)),
        )
        .style(Style::new().white())
        .alignment(Alignment::Left);

    frame.render_widget(p, layout[0]);
    render_status_line(model, frame, layout[1]);
}

fn render_status_line(model: &Model, frame: &mut Frame, rect: Rect) {
    let mut status_bar_lines = vec![Span::styled(
        TITLE,
        Style::new()
            .bold()
            .bg(PRIMARY_COLOR)
            .fg(SECTION_TITLE_FG_COLOR),
    )];

    status_bar_lines.push(Span::from(format!(" [{}]", &model.profile_name)).fg(SECONDARY_COLOR));

    if model.auto_refresh {
        if model.num_services_marked_for_refresh == 0 {
            status_bar_lines
                .push(Span::from(" auto refresh on for all services").fg(MESSAGE_COLOR));
        } else if model.num_services_marked_for_refresh == 1 {
            status_bar_lines.push(Span::from(" auto refresh on for 1 service").fg(MESSAGE_COLOR));
        } else {
            status_bar_lines.push(
                Span::from(format!(
                    " auto refresh on for {} services",
                    model.num_services_marked_for_refresh
                ))
                .fg(MESSAGE_COLOR),
            );
        }
    }

    if model.debug {
        if model.num_errors > 0 {
            status_bar_lines.push(Span::from(format!(" [{} errors]", model.num_errors)));
        }
        status_bar_lines.push(Span::from(
            model
                .last_active_pane
                .map(|p| format!(" [{}]", p))
                .unwrap_or(" -".to_string()),
        ));
        status_bar_lines.push(Span::from(format!(" -> [{}]", model.active_pane)));
        status_bar_lines.push(Span::from(format!(
            " [render counter: {}]",
            model.render_counter
        )));
        status_bar_lines.push(Span::from(format!(
            " [event counter: {}]",
            model.event_counter
        )));

        status_bar_lines.push(Span::from(format!(
            " [dimensions: {}x{}] ",
            model.terminal_dimensions.width, model.terminal_dimensions.height
        )));
    }

    if let Some(msg) = &model.user_message {
        let span = match msg {
            UserMessage::Info(m, _) => {
                Span::styled(format!(" {}", m), Style::new().fg(INFO_MESSAGE_COLOR))
            }
            UserMessage::Error(m, _) => {
                Span::styled(format!(" {}", m), Style::new().fg(ERROR_MESSAGE_COLOR))
            }
        };

        status_bar_lines.push(span);
    }

    let status_bar_text = Line::from(status_bar_lines);

    let status_bar = Paragraph::new(status_bar_text).block(Block::default());

    frame.render_widget(&status_bar, rect);
}

fn render_services_list(model: &mut Model, frame: &mut Frame, rect: Rect) {
    let title = " services ";

    let (border_color, title_color, highlight_color) = if model.active_pane == Pane::ServicesList {
        (PRIMARY_COLOR, PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (
            INACTIVE_PANE_BORDER_COLOR,
            INACTIVE_PANE_TITLE_BG_COLOR,
            INACTIVE_PANE_SELECTED_COLOR,
        )
    };

    let items: Vec<ListItem> = model
        .service_items
        .items
        .iter()
        .map(ListItem::from)
        .collect();

    if items.is_empty() {
        let details = Paragraph::new("services will appear here")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(border_color))
                    .title_style(
                        Style::new()
                            .bold()
                            .bg(title_color)
                            .fg(SECTION_TITLE_FG_COLOR),
                    )
                    .title(" tasks ")
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left);

        frame.render_widget(&details, rect);

        return;
    }

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .padding(Padding::new(0, 0, 1, 0))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(title_color)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(title),
        )
        .style(Style::new().white())
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true)
        .highlight_style(Style::new().fg(highlight_color))
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, rect, &mut model.service_items.state);
}

fn render_service_details(model: &Model, frame: &mut Frame, rect: Rect) {
    let color = if model.active_pane == Pane::ServiceDetails {
        PRIMARY_COLOR
    } else {
        INACTIVE_PANE_BORDER_COLOR
    };

    let maybe_selected = model.service_items.state.selected();

    if let Some(selected) = maybe_selected {
        let maybe_service_item = model.service_items.items.get(selected);
        if let Some(service_item) = maybe_service_item {
            let (colorr, title) = if service_item.service.is_ok() {
                (color, "details")
            } else {
                (ERROR_MESSAGE_COLOR, "error")
            };
            let details = match &service_item.service {
                Ok(service) => format!(
                    r#"
Name             {}
Cluster Keys     {:?}
Cluster ARN      {}
Status           {}
Desired count    {}
Running count    {}
Pending count    {}
"#,
                    &service.name,
                    &service.cluster_keys,
                    if model.redact_mode {
                        REDACTED
                    } else {
                        &service.cluster_arn
                    },
                    &service.status,
                    &service.desired_count,
                    &service.running_count,
                    &service.pending_count,
                ),

                Err(err) => format!(
                    r#"
Error            {}
"#,
                    err.error
                ),
            };
            let details = Paragraph::new(details)
                .block(
                    Block::bordered()
                        .border_style(Style::default().fg(colorr))
                        .title(title)
                        .padding(Padding::new(1, 0, 0, 0)),
                )
                .style(Style::new().white().on_black())
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Left);

            frame.render_widget(&details, rect);
        };
    } else {
        let details = Paragraph::new("")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(color))
                    .title("details")
                    .padding(Padding::new(1, 0, 0, 0)),
            )
            .style(Style::new().white().on_black())
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left);

        frame.render_widget(&details, rect);
    }
}

fn render_tasks_list(model: &mut Model, frame: &mut Frame, rect: Rect) {
    let (color, highlight_color) = if model.active_pane == Pane::TasksList {
        (PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (INACTIVE_PANE_BORDER_COLOR, INACTIVE_PANE_SELECTED_COLOR)
    };

    match &mut model.task_items {
        Some(i) => {
            let items: Vec<ListItem> = i.items.iter().map(ListItem::from).collect();

            let list = List::new(items)
                .block(
                    Block::bordered()
                        .border_style(Style::default().fg(color))
                        .padding(Padding::new(0, 0, 1, 0))
                        .title_style(Style::new().bold().bg(color).fg(SECTION_TITLE_FG_COLOR))
                        .title(" tasks "),
                )
                .style(Style::new().white())
                .highlight_symbol("> ")
                .repeat_highlight_symbol(true)
                .highlight_style(Style::new().fg(highlight_color))
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(list, rect, &mut i.state);
        }
        None => {
            let details = Paragraph::new("tasks will appear here")
                .block(
                    Block::bordered()
                        .border_style(Style::default().fg(color))
                        .title_style(Style::new().bold().bg(color).fg(SECTION_TITLE_FG_COLOR))
                        .title(" tasks ")
                        .padding(Padding::new(1, 0, 1, 0)),
                )
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left);

            frame.render_widget(&details, rect);
        }
    }
}

fn render_task_details(model: &Model, frame: &mut Frame, rect: Rect) {
    let color = if model.active_pane == Pane::TaskDetails {
        PRIMARY_COLOR
    } else {
        INACTIVE_PANE_BORDER_COLOR
    };

    let selected_task = match model.get_selected_task() {
        Some(t) => t,
        None => {
            let paragraph = Paragraph::new("")
                .block(
                    Block::bordered()
                        .border_style(Style::default().fg(color))
                        .title("details")
                        .padding(Padding::new(1, 0, 0, 0)),
                )
                .style(Style::new().white().on_black())
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Left);

            frame.render_widget(&paragraph, rect);

            return;
        }
    };

    let details = format!(
        r#"
ARN              {}
Health status    {}
CPU              {}
Memory           {}
Last Status      {}
"#,
        if model.redact_mode {
            REDACTED
        } else {
            selected_task.task_arn().unwrap_or(UNKNOWN_VALUE)
        },
        selected_task
            .health_status()
            .map(|s| s.as_str())
            .unwrap_or(UNKNOWN_VALUE),
        selected_task.cpu().unwrap_or(UNKNOWN_VALUE),
        selected_task.memory().unwrap_or(UNKNOWN_VALUE),
        selected_task.last_status().unwrap_or(UNKNOWN_VALUE),
    );

    let paragraph = Paragraph::new(details)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(color))
                .title("details")
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .style(Style::new().white().on_black())
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(&paragraph, rect);
}

fn render_containers_list(model: &mut Model, frame: &mut Frame, rect: Rect) {
    let (color, highlight_color) = if model.active_pane == Pane::ContainersList {
        (PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (INACTIVE_PANE_BORDER_COLOR, INACTIVE_PANE_SELECTED_COLOR)
    };

    if model.task_items.is_none() {
        let details = Paragraph::new("containers will appear here")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(color))
                    .title_style(Style::new().bold().bg(color).fg(SECTION_TITLE_FG_COLOR))
                    .title(" containers ")
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left);

        frame.render_widget(&details, rect);
    } else {
        let list = List::new(
            model
                .container_items
                .items
                .iter()
                .map(ListItem::from)
                .collect::<Vec<_>>(),
        )
        .block(
            Block::bordered()
                .border_style(Style::default().fg(color))
                .padding(Padding::new(0, 0, 1, 0))
                .title_style(Style::new().bold().bg(color).fg(SECTION_TITLE_FG_COLOR))
                .title(" containers "),
        )
        .style(Style::new().white())
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true)
        .highlight_style(Style::new().fg(highlight_color))
        .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, rect, &mut model.container_items.state);
    }
}

fn render_container_details(model: &Model, frame: &mut Frame, rect: Rect) {
    let border_color = if model.active_pane == Pane::ContainerDetails {
        PRIMARY_COLOR
    } else {
        INACTIVE_PANE_BORDER_COLOR
    };

    if model.task_items.is_none() {
        let details = Paragraph::new("")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(border_color))
                    .title("details")
                    .padding(Padding::new(1, 0, 0, 0)),
            )
            .style(Style::new().white().on_black())
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left);

        frame.render_widget(&details, rect);

        return;
    }

    let maybe_selected = model.container_items.state.selected();

    if let Some(selected) = maybe_selected {
        let maybe_container_item = model.container_items.items.get(selected);
        if let Some(container_item) = maybe_container_item {
            let details = format!(
                r#"
Image            {}
Last Status      {}
CPU              {}
Memory           {}
Health Status    {}
"#,
                if model.redact_mode {
                    REDACTED
                } else {
                    container_item.container.image().unwrap_or(UNKNOWN_VALUE)
                },
                container_item
                    .container
                    .last_status()
                    .unwrap_or(UNKNOWN_VALUE),
                container_item.container.cpu().unwrap_or(UNKNOWN_VALUE),
                container_item.container.memory().unwrap_or(UNKNOWN_VALUE),
                container_item
                    .container
                    .health_status()
                    .map(|s| s.as_str())
                    .unwrap_or(UNKNOWN_VALUE),
            );
            let details = Paragraph::new(details)
                .block(
                    Block::bordered()
                        .border_style(Style::default().fg(border_color))
                        .title("details")
                        .padding(Padding::new(1, 0, 0, 0)),
                )
                .style(Style::new().white().on_black())
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Left);

            frame.render_widget(&details, rect);
        };
    } else {
        let details = Paragraph::new("container details will appear here")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(border_color))
                    .title("details")
                    .padding(Padding::new(1, 0, 0, 0)),
            )
            .style(Style::new().white().on_black())
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left);

        frame.render_widget(&details, rect);
    }
}

fn render_list_view(model: &mut Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Min(10),
            Constraint::Min(6),
            Constraint::Min(6),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let service_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[0]);

    let task_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[1]);

    let container_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[2]);

    render_services_list(model, frame, service_layout[0]);
    render_service_details(model, frame, service_layout[1]);
    render_tasks_list(model, frame, task_layout[0]);
    render_task_details(model, frame, task_layout[1]);
    render_containers_list(model, frame, container_layout[0]);
    render_container_details(model, frame, container_layout[1]);
    render_status_line(model, frame, layout[3]);
}
