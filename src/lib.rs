use leptos::*;

pub mod task {
    #[derive(Debug, Clone, Copy)]
    pub enum TaskState {
        NotStarted,
        Done,
    }

    #[derive(Debug, Clone)]
    pub struct Task {
        pub state: TaskState,
        pub text: String,
    }

    impl Task {
        pub fn new(text: &str) -> Self {
            Task {
                state: TaskState::NotStarted,
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
        pub fn new(cx: Scope, text: &str, max_duration: Duration) -> Self {
            Self {
                state: create_rw_signal(cx, TimedTaskState::NotStarted),
                text: text.to_string(),
                max_duration,
                duration_remaining: create_rw_signal(cx, max_duration),
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