use super::common::Pane;
use crate::domain::{ServiceDetails, ServiceResult};
use aws_sdk_ecs::types::Task;

pub enum Message {
    TerminalResize(u16, u16),
    GoToNextListItem,
    GoToPreviousListItem,
    GoToFirstListItem,
    GoToLastListItem,
    ServicesFetched(Vec<ServiceResult>),
    ServiceDetailsRefreshed((ServiceResult, ServiceDetails, usize)),
    TasksFetched((ServiceDetails, Vec<Task>, bool)),
    ClearUserMsg,
    RefreshResultsForMarkedServices,
    RefreshResultsForCurrentItem,
    ToggleServiceRefresh,
    ToggleAutoRefresh,
    GoBackOrQuit,
    QuitImmediately,
    GoToPane(Pane),
}
