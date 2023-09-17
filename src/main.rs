use std::time::Duration;
// Note! std::time::Instant is *not* implemented on WASM!
use web_time::Instant;

use leptos::{logging::log, *};

use api_freshwater_master_test_kit_ui::{
    task::{Task, TaskState},
    timed_task::{TimedTask, TimedTaskState},
};

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let item_lists = [
        ItemList::new(
            "pH",
            vec![
                PreListItem::Task(Task::new("Fill test tube with 5ml of water")),
                PreListItem::Task(Task::new("Add 3 drops of pH Test Solution")),
                PreListItem::TimedTask(TimedTask::new(
                    "Shake test tube for",
                    Duration::from_secs(5),
                )),
                PreListItem::TimedTask(TimedTask::new("Wait for", Duration::from_secs(5 * 60))),
                PreListItem::Input(Input::new()),
            ],
        ),
        ItemList::new(
            "Ammonia",
            vec![
                PreListItem::Task(Task::new("Fill test tube with 5ml of water")),
                PreListItem::Task(Task::new("Add 8 drops from Ammonia Test Solution #1")),
                PreListItem::Task(Task::new("Add 8 drops from Ammonia Test Solution #2")),
                PreListItem::TimedTask(TimedTask::new(
                    "Shake test tube for",
                    Duration::from_secs(5),
                )),
                PreListItem::TimedTask(TimedTask::new("Wait for", Duration::from_secs(5 * 60))),
                PreListItem::Input(Input::new()),
            ],
        ),
        ItemList::new(
            "Nitrite",
            vec![
                PreListItem::Task(Task::new("Fill test tube with 5ml of water")),
                PreListItem::Task(Task::new("Add 5 drops of Nitrite Test Solution")),
                PreListItem::TimedTask(TimedTask::new(
                    "Shake test tube for",
                    Duration::from_secs(5),
                )),
                PreListItem::TimedTask(TimedTask::new("Wait for", Duration::from_secs(5 * 60))),
                PreListItem::Input(Input::new()),
            ],
        ),
        ItemList::new(
            "Nitrate",
            vec![
                PreListItem::Task(Task::new("Fill test tube with 5ml of water")),
                PreListItem::Task(Task::new("Add 10 drops of Nitrate Test Solution #1")),
                PreListItem::Task(Task::new(
                    "Cap the test tube & invert tube several times to mix solution",
                )),
                PreListItem::TimedTask(TimedTask::new(
                    "Shake Nitrate Test Solution #2 for",
                    Duration::from_secs(30),
                )),
                PreListItem::Task(Task::new("Add 10 drops of Nitrate Test Solution #2")),
                PreListItem::TimedTask(TimedTask::new(
                    "Shake test tube for",
                    Duration::from_secs(60),
                )),
                PreListItem::TimedTask(TimedTask::new("Wait for", Duration::from_secs(5 * 60))),
                PreListItem::Input(Input::new()),
            ],
        ),
    ];

    item_lists
        .into_iter()
        .map(|item_list| {
            view! {<ItemListComponent item_list/>}
        })
        .collect_view()
}

#[component]
fn ItemListComponent(item_list: ItemList) -> impl IntoView {
    let item_list_context = ItemListContext {
        pre_list_items: item_list.pre_list_items,
        items: item_list.items,
    };

    view! {
        <div class="item_list">
            <div class="item_list_header">
                <h2>{item_list.name}</h2>
                <button on:click=move |_| {item_list.items.update(|l| {l.pop();})}>undo</button>
            </div>
            <For
                each=item_list.items
                key=|&(idx, _)| idx
                view=move |(_, item)| {
                    view! {<ListItemComponent item item_list_context/>}
                }
            />
        </div>
    }
}

#[component]
fn ListItemComponent(item: ListItem, item_list_context: ItemListContext) -> impl IntoView {
    match item {
        ListItem::TimedTask(timed_task) => {
            view! {<TimedTaskComponent timed_task item_list_context/>}
        }
        ListItem::Task(task) => {
            view! {<TaskComponent task item_list_context/>}
        }
        ListItem::Input(input) => {
            view! {<InputComponent input item_list_context/>}
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ItemListContext {
    pre_list_items: ReadSignal<Vec<PreListItem>>,
    items: RwSignal<Vec<(usize, ListItem)>>,
}

#[component]
fn TimedTaskComponent(
    timed_task: RwSignal<TimedTask>,
    item_list_context: ItemListContext,
) -> impl IntoView {
    view! { <div>
        <div>{timed_task.get_untracked().text} {move || format!(" {:#.2?}", timed_task.get_untracked().duration_remaining.get())}</div>
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
                TimedTaskState::Done => {
                    ItemList::try_add_item(
                        &item_list_context.pre_list_items.get_untracked(),
                        item_list_context.items
                    );
                },
            }
        }>{move || match timed_task.get_untracked().state.get() {
            TimedTaskState::NotStarted => "start",
            TimedTaskState::Ongoing => "reset",
            TimedTaskState::Done => "next",
        }}</button>
        </div>
    }
}

#[component]
fn TaskComponent(task: RwSignal<Task>, item_list_context: ItemListContext) -> impl IntoView {
    view! { <div>
        <div>{move || task.get_untracked().text}</div>
        <button on:click=move|_| {
            task.get_untracked().state.update(|t| {
                *t = match *t {
                    TaskState::NotStarted => TaskState::InProgress,
                    TaskState::InProgress => {
                        ItemList::try_add_item(
                            &item_list_context.pre_list_items.get_untracked(),
                            item_list_context.items
                        );
                        TaskState::Done
                    }
                    TaskState::Done => {
                        TaskState::Done
                    },
                }
            });
        }>{
            move || task.get_untracked().state.with(|t| {
                match *t {
                    TaskState::NotStarted => "start",
                    TaskState::InProgress => "next",
                    TaskState::Done => "next",
                }
            })
        }</button>
        </div>
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
    Input(RwSignal<Input>),
}

// I still don't understand how leptos author was able to remove cx, makes writing `From` derivatives 10x easier ty
impl From<PreListItem> for ListItem {
    fn from(pre_list_item: PreListItem) -> Self {
        match pre_list_item {
            PreListItem::TimedTask(timed_task) => ListItem::TimedTask(create_rw_signal(timed_task)),
            PreListItem::Task(task) => ListItem::Task(create_rw_signal(task)),
            PreListItem::Input(input) => ListItem::Input(create_rw_signal(input)),
        }
    }
}

#[component]
fn InputComponent(input: RwSignal<Input>, item_list_context: ItemListContext) -> impl IntoView {
    view! {
        <input type="text"
            on:input=move |ev| {
                input.get().value.set(event_target_value(&ev))
            }
            prop:value=input.get().value
        />
    }
}

#[derive(Debug, Clone)]
struct Input {
    value: RwSignal<String>,
}

impl Input {
    fn new() -> Self {
        Self {
            value: create_rw_signal(String::new()),
        }
    }
}

#[derive(Debug)]
struct ItemList {
    name: ReadSignal<String>,
    pre_list_items: ReadSignal<Vec<PreListItem>>,
    items: RwSignal<Vec<(usize, ListItem)>>,
}

enum AddItemToItemListResult {
    Success,
    Failure,
}

impl ItemList {
    fn new(name: &str, pre_list_items: Vec<PreListItem>) -> Self {
        let mut items = vec![];
        if !pre_list_items.is_empty() {
            items.push((0usize, ListItem::from(pre_list_items[0].clone())));
        }
        let items = create_rw_signal(items);
        Self {
            name: create_signal(name.to_string()).0,
            items,
            pre_list_items: create_signal(pre_list_items).0,
        }
    }

    fn try_add_item(
        pre_list_items: &[PreListItem],
        items: RwSignal<Vec<(usize, ListItem)>>,
    ) -> AddItemToItemListResult {
        if items.get_untracked().len() == pre_list_items.len() {
            AddItemToItemListResult::Failure
        } else {
            let index_to_add_from_pre_list_items = items.get_untracked().len();
            items.update(|items| {
                items.push((
                    index_to_add_from_pre_list_items,
                    ListItem::from(pre_list_items[index_to_add_from_pre_list_items].clone()),
                ));
            });
            AddItemToItemListResult::Success
        }
    }
}

#[component]
fn TaskList(name: String, tasks: Vec<TimedTask>) -> impl IntoView {}

#[component]
fn Task() -> impl IntoView {}
