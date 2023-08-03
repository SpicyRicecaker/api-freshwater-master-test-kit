use std::time::Duration;
// Note! std::time::Instant is *not* implemented on WASM!
use web_time::Instant;

use leptos::*;

use api_freshwater_master_test_kit_ui::{task::{Task, TaskState}, timed_task::{TimedTask, TimedTaskState}};

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
                    // Duration::from_secs(5 * 60),
                    Duration::from_secs(1),
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
                                    ListItem::TimedTask((timed_task, _)) => {
                                        view! { cx,
                                            <div>
                                                <button on:click=move|_| {
                                                    match timed_task.get_untracked().state.get() {
                                                        TimedTaskState::NotStarted => {
                                                            // change the state
                                                            timed_task.get_untracked().state.update(|s| *s = TimedTaskState::Ongoing);

                                                            // start the timer.
                                                            fn tick (previous_instant: Instant, timed_task: TimedTask) {
                                                                let now = Instant::now();

                                                                timed_task.duration_remaining.update(|d| *d = d.saturating_sub(previous_instant.elapsed()));
                                                                // only go on if
                                                                // 1) duration isn't 0
                                                                if timed_task.duration_remaining.get_untracked() == Duration::ZERO {
                                                                    timed_task.state.update(|s| *s = TimedTaskState::Done);
                                                                    return;
                                                                }

                                                                // 2) we haven't changed state to NotStarted
                                                                if timed_task.state.get_untracked() == TimedTaskState::NotStarted {
                                                                    // reset duration to max duration
                                                                    timed_task.duration_remaining.update(|d| *d = timed_task.max_duration);
                                                                    return;
                                                                }

                                                                request_animation_frame(move || tick(now, timed_task));
                                                            }
                                                            let now = Instant::now();
                                                            request_animation_frame(move || tick(now, timed_task.get_untracked()));
                                                        },
                                                        // WIP maybe add some way to undo a reset?
                                                        TimedTaskState::Ongoing => {
                                                            // pause the timer
                                                            timed_task.get_untracked().state.update(|s| *s = TimedTaskState::NotStarted);
                                                        },
                                                        // maybe hide the button completely
                                                        TimedTaskState::Done => todo!(),
                                                    }
                                                }>{move || match timed_task.get_untracked().state.get() {
                                                    TimedTaskState::NotStarted => "start",
                                                    TimedTaskState::Ongoing => "reset",
                                                    TimedTaskState::Done => "next",
                                                }}</button>
                                                <div>{move || timed_task.get().text} {move || format!(" {:#.2?}", timed_task.get().duration_remaining.get())}</div>
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
