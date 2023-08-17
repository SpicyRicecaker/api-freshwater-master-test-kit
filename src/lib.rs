use leptos::*;
pub const IS_TEST: bool = true;

pub mod task {
    use super::*;
    #[derive(Debug, Clone, Copy)]
    pub enum TaskState {
        NotStarted,
        Done,
    }

    #[derive(Debug, Clone)]
    pub struct Task {
        pub state: RwSignal<TaskState>,
        pub text: String,
    }

    impl Task {
        pub fn new(text: &str) -> Self {
            Task {
                state: create_rw_signal(TaskState::NotStarted),
                text: text.to_string(),
            }
        }
    }
}

pub mod timed_task {
    use std::time::Duration;

    use super::*;
    /// Note: all struct fields which aren't signals are basically immutable
    #[derive(Debug, Clone)]
    pub struct TimedTask {
        pub state: RwSignal<TimedTaskState>,
        pub text: String,
        pub max_duration: Duration,
        pub duration_remaining: RwSignal<Duration>,
    }

    impl TimedTask {
        pub fn new(text: &str, mut max_duration: Duration) -> Self {
            if IS_TEST {
                max_duration = Duration::from_millis(300);
            }
            Self {
                state: create_rw_signal(TimedTaskState::NotStarted),
                text: text.to_string(),
                max_duration,
                duration_remaining: create_rw_signal(max_duration),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TimedTaskState {
        NotStarted,
        Ongoing,
        Done,
    }
}
