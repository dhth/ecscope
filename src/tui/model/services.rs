use crate::domain::*;
use ratatui::{
    text::Line,
    widgets::{ListItem, ListState},
};

#[derive(Debug)]
pub struct ServiceItem {
    pub service: ServiceResult,
    pub marked_for_refresh: bool,
}

impl ServiceItem {
    pub fn new(service: ServiceResult) -> Self {
        Self {
            service,
            marked_for_refresh: false,
        }
    }
}

#[derive(Debug)]
pub struct ServiceItems {
    pub items: Vec<ServiceItem>,
    pub state: ListState,
}

impl From<&Vec<ServiceResult>> for ServiceItems {
    fn from(services: &Vec<ServiceResult>) -> Self {
        let mut items: Vec<ServiceItem> = services
            .iter()
            .map(|service| ServiceItem::new(service.clone()))
            .collect();

        items.sort_by(|a, b| match &a.service {
            Ok(sa) => match &b.service {
                Ok(sb) => sa.name.cmp(&sb.name),
                Err(_) => std::cmp::Ordering::Less,
            },
            Err(_) => std::cmp::Ordering::Greater,
        });

        let mut state = ListState::default();
        if !services.is_empty() {
            state.select(Some(0))
        }

        Self { items, state }
    }
}

impl ServiceItems {
    pub fn append(&mut self, services: &[ServiceResult]) {
        let mut new_items = services
            .iter()
            .map(|service| ServiceItem::new(service.clone()))
            .collect();

        self.items.append(&mut new_items);

        self.items.sort_by(|a, b| match &a.service {
            Ok(sa) => match &b.service {
                Ok(sb) => sa.name.cmp(&sb.name),
                Err(_) => std::cmp::Ordering::Less,
            },
            Err(_) => std::cmp::Ordering::Greater,
        });

        let selected = match self.state.selected() {
            Some(i) => Some(i),
            None => {
                if self.items.is_empty() {
                    None
                } else {
                    Some(0)
                }
            }
        };

        self.state = self.state.clone().with_selected(selected);
    }
}

impl ServiceItems {
    pub fn empty() -> Self {
        Self::from(&Vec::new())
    }
}

impl From<&ServiceItem> for ListItem<'_> {
    fn from(value: &ServiceItem) -> Self {
        let line = match &value.service {
            Ok(service_details) => {
                let refresh_marker = match value.marked_for_refresh {
                    true => "* ",
                    false => "  ",
                };
                let pending_marker = if (service_details.desired_count
                    != service_details.running_count)
                    || service_details.pending_count != 0
                {
                    "~ "
                } else {
                    "  "
                };

                let identifier = if !service_details.cluster_keys.is_empty() {
                    service_details.cluster_keys[0]
                        .get(..5)
                        .unwrap_or(service_details.cluster_keys[0].as_str())
                } else {
                    ""
                };

                Line::from(format!(
                    "{:6}{}{}{}",
                    identifier, refresh_marker, pending_marker, service_details.name
                ))
            }
            Err(service_error) => {
                let identifier = if !service_error.cluster_keys.is_empty() {
                    service_error.cluster_keys[0]
                        .get(..5)
                        .unwrap_or(service_error.cluster_keys[0].as_str())
                } else {
                    ""
                };

                Line::from(format!(
                    "{:6}  x {}",
                    identifier, service_error.service_name
                ))
            }
        };
        ListItem::new(line)
    }
}
