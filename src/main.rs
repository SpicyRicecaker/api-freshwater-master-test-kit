use std::time::Duration;
// Note! std::time::Instant is *not* implemented on WASM!
use web_time::Instant;

use leptos::*;

use api_freshwater_master_test_kit_ui::{
    task::{Task, TaskState},
    timed_task::{TimedTask, TimedTaskState},
};

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
                PreListItem::Task(Task::new(cx, "Add 5 drops")),
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
                                    ListItem::TimedTask(timed_task) => {
                                        view! { cx,
                                            <div>
                                                <button on:click=move |_|{
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
                                                <div>{timed_task.get_untracked().text} {move || format!(" {:#.2?}", timed_task.get_untracked().duration_remaining.get())}</div>
                                            </div>
                                        }.into_any()
                                    },
                                    ListItem::Task(task) => {
                                        view! { cx,
                                            <div>
                                                <button on:click=move|_| {
                                                    task.get_untracked().state.update(|t| {
                                                        log!("clicked. state: {:#?}", t);
                                                        *t = match *t {
                                                            TaskState::NotStarted => TaskState::Done,
                                                            TaskState::Done => TaskState::Done,
                                                        }
                                                    });
                                                }>{
                                                    move || task.get_untracked().state.with(|t| {
                                                        match *t {
                                                            TaskState::NotStarted => "start",
                                                            TaskState::Done => "DONE",
                                                        }
                                                    })
                                                }</button>
                                                <div>{move || task.get_untracked().text}</div>
                                            </div>
                                        }.into_any()
                                    }
                                    ListItem::Input(input) => {
                                        todo!()
                                    },
                                }}
                            }
                        }
                    />
                </div>
            }
        }).collect_view(cx)
        }
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
    TimedTask(RwSignal<TimedTask>),
    Task(RwSignal<Task>),
    Input(Input),
}

#[derive(Debug, Clone)]
struct Input {}

struct ItemList {
    name: String,
    items: RwSignal<Vec<(usize, ListItem)>>,
}

impl ItemList {
    fn new(cx: Scope, name: &str, pre_items: Vec<PreListItem>) -> Self {
        let items = create_rw_signal(
            cx,
            pre_items
                .into_iter()
                .enumerate()
                .map(|(idx, pre_item)| {
                    (
                        idx,
                        match pre_item {
                            PreListItem::TimedTask(timed_task) => {
                                ListItem::TimedTask(create_rw_signal(cx, timed_task))
                            }
                            PreListItem::Task(task) => ListItem::Task(create_rw_signal(cx, task)),
                            PreListItem::Input(i) => ListItem::Input(i),
                        },
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            name: name.to_string(),
            items,
        }
    }
}

#[component]
fn TaskList(cx: Scope, name: String, tasks: Vec<TimedTask>) -> impl IntoView {}

#[component]
fn Task(cx: Scope) -> impl IntoView {}
