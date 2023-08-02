use std::time::{Duration, Instant};

use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let item_lists = [
        ItemList::new(
            cx,
            "pH",
            vec![
                PreListItem::Task(Task::new("Add 5 drops")),
                PreListItem::TimedTask(TimedTask::new(
                    cx,
                    "Shake for",
                    Duration::from_secs(5 * 60),
                )),
            ],
        ),
        ItemList::new(cx, "Ammonia", vec![]),
        ItemList::new(cx, "Nirite", vec![]),
        ItemList::new(cx, "Nitrate", vec![]),
    ];
    view! {cx,
        {item_lists.into_iter().map(|item_list| {
            view! {cx,
                <div class="item_list">
                    <h2>{item_list.name}</h2>
                    <For
                        each=item_list.items
                        key=|(idx, _)| *idx
                        view=move |cx, (_, item)| {
                            view! { cx,
                                {match item {
                                    ListItem::TimedTask((timed_task, set_timed_task)) => {
                                        view! { cx,
                                            <div>
                                                <button on:click=move|_| {
                                                    match timed_task.get().state {
                                                        TimedTaskState::NotStarted => {
                                                            // start the timer.
                                                            let tick = move || {
                                                                dbg!(timed_task.get().duration_remaining);
                                                                let lol = move || {

                                                                };
                                                                request_animation_frame(lol);
                                                            };
                                                            // |mut prev| {
                                                            //     if timed_tast.get().state == TimedTaskState::Ongoing {
                                                            //         //
                                                            //     }
                                                            // }

                                                            // change the state
                                                            set_timed_task.update(|t| t.state = TimedTaskState::Ongoing);
                                                        },
                                                        // WIP maybe add some way to undo a reset?
                                                        TimedTaskState::Ongoing => {
                                                            todo!()
                                                        },
                                                        // maybe hide the button completely
                                                        TimedTaskState::Done => todo!(),
                                                    }
                                                }>{move || match timed_task.get().state {
                                                    TimedTaskState::NotStarted => "start",
                                                    TimedTaskState::Ongoing => "reset",
                                                    TimedTaskState::Done => "",
                                                }}</button>
                                                <div>{move || timed_task.get().text} {move || format!(" {:#?}", timed_task.get().duration_remaining)}</div>
                                            </div>
                                        }.into_any()
                                    },
                                    ListItem::Task((task, set_task)) => {
                                        view! { cx,
                                            <div>
                                                <button on:click=move|_| {set_task.update(|t| {
                                                    t.state = match t.state {
                                                        TaskState::NotStarted => TaskState::Done,
                                                        TaskState::Done => TaskState::Done,
                                                    };
                                                })}>{move || match task.get().state {
                                                    TaskState::NotStarted => "start",
                                                    TaskState::Done => "DONE",
                                                }}</button>
                                                <div>{move || task.get().text}</div>
                                            </div>
                                        }.into_any()
                                    },
                                    ListItem::Input((input, set_input)) => {
                                        todo!()
                                    },
                                }}
                            }
                        }
                    />
                </div>
            }
        }).collect_view(cx)}
    }
}

#[derive(Debug, Clone)]
enum PreListItem {
    TimedTask(TimedTask),
    Task(Task),
    Input(Input),
}

#[derive(Debug, Clone)]
enum ListItem {
    TimedTask((ReadSignal<TimedTask>, WriteSignal<TimedTask>)),
    Task((ReadSignal<Task>, WriteSignal<Task>)),
    Input((ReadSignal<Input>, WriteSignal<Input>)),
}

#[derive(Debug, Clone)]
enum TimedTaskState {
    NotStarted,
    Ongoing,
    Done,
}

#[derive(Debug, Clone, Copy)]
enum TaskState {
    NotStarted,
    Done,
}

#[derive(Debug, Clone)]
struct TimedTask {
    state: TimedTaskState,
    text: String,
    max_duration: Duration,
    duration_remaining: (ReadSignal<Duration>, WriteSignal<Duration>),
}

impl TimedTask {
    fn new(cx: Scope, text: &str, max_duration: Duration) -> Self {
        Self {
            state: TimedTaskState::NotStarted,
            text: text.to_string(),
            max_duration,
            duration_remaining: create_signal(cx, max_duration),
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    state: TaskState,
    text: String,
}

impl Task {
    fn new(text: &str) -> Self {
        Task {
            state: TaskState::NotStarted,
            text: text.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {}

struct ItemList {
    name: String,
    items: ReadSignal<Vec<(usize, ListItem)>>,
    set_items: WriteSignal<Vec<(usize, ListItem)>>,
}

impl ItemList {
    fn new(cx: Scope, name: &str, pre_list_items: Vec<PreListItem>) -> Self {
        let (items, set_items) = create_signal(
            cx,
            pre_list_items
                .into_iter()
                .enumerate()
                .map(|(i, pre_list_item)| {
                    (i, {
                        match pre_list_item {
                            PreListItem::Input(input) => ListItem::Input(create_signal(cx, input)),
                            PreListItem::TimedTask(timed_task) => {
                                ListItem::TimedTask(create_signal(cx, timed_task))
                            }
                            PreListItem::Task(task) => ListItem::Task(create_signal(cx, task)),
                        }
                    })
                })
                .collect::<Vec<_>>(),
        );
        Self {
            name: name.to_string(),
            items,
            set_items,
        }
    }
}

#[component]
fn TaskList(cx: Scope, name: String, tasks: Vec<TimedTask>) -> impl IntoView {}

#[component]
fn Task(cx: Scope) -> impl IntoView {}
