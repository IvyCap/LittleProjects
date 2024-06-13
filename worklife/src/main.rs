#![allow(non_snake_case)]
#[warn(deprecated)]

use dioxus::prelude::*;
use tracing::Level;

const HOURSDAY:f32 = 24.0;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        // #[route("/Percentage")]
        // Percentage {counter: Counter},
        #[route("/Settings")]
        Settings {},
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
struct Counter {
    sleep: Signal<f32>,
    walk: Signal<f32>,
    exer: Signal<f32>,
    hobby: Signal<f32>,
    learn: Signal<f32>,
    work_self: Signal<f32>,
    work_man: Signal<f32>,
}

impl Counter {
    fn new() -> Counter {
    Counter { 
        sleep: use_signal(|| 0.0),
        walk: use_signal(|| 0.0),
        exer: use_signal(|| 0.0),
        hobby: use_signal(|| 0.0),
        learn: use_signal(|| 0.0),
        work_self: use_signal(|| 0.0),
        work_man: use_signal(|| 0.0),
        }
    }

    pub fn daily_percent(mut self, counter: &Counter) -> Self {
        self.sleep =  use_signal(|| (counter.sleep / 24.0) * 100.0);
        self.walk = use_signal(|| (counter.walk / 24.0) * 100.0);
        self.exer = use_signal(|| (counter.exer / 24.0) * 100.0);
        self.hobby = use_signal(|| (counter.hobby / 24.0) * 100.0);
        self.learn = use_signal(|| (counter.learn / 24.0) * 100.0);
        self.work_self = use_signal(|| (counter.work_self / 24.0) * 100.0);
        self.work_man = use_signal(|| (counter.work_man / 24.0) * 100.0);
        self
    }

    pub fn daily_percent2(mut self) -> Self {
        self.sleep =  use_signal(|| (self.sleep / 24.0) * 100.0);
        self.walk = use_signal(|| (self.walk / 24.0) * 100.0);
        self.exer = use_signal(|| (self.exer / 24.0) * 100.0);
        self.hobby = use_signal(|| (self.hobby / 24.0) * 100.0);
        self.learn = use_signal(|| (self.learn / 24.0) * 100.0);
        self.work_self = use_signal(|| (self.work_self / 24.0) * 100.0);
        self.work_man = use_signal(|| (self.work_man / 24.0) * 100.0);
        self
    }
}


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}


#[component]
fn NavBar() -> Element {
    rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            // Link { to: Route::Percentage {counter}, "Daily Percent" }
            Link { to: Route::Settings {}, "Settings" }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn App() -> Element {

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
    }
}

// #[component]
// fn Percentage(counter: Counter) -> Element {
//     let sleep_pert =  (counter.sleep / HOURSDAY) * 100;
//     let walk_pert = (counter.walk / HOURSDAY) * 100;
//     let exer_pert = (counter.exer / HOURSDAY) * 100;
//     let hobby_pert = (counter.hobby / HOURSDAY) * 100;
//     let learn_pert = (counter.learn / HOURSDAY) * 100;
//     let work_self_pert = (counter.work_self / HOURSDAY) * 100;
//     let work_man_pert = (counter.work_man / HOURSDAY) * 100;

//     rsx! {

//         div{
//             h1 { "Balance Percentage" }
//             h3 {"Sleeping Percentage: {sleep_pert}%"}
//             h3 {"Sleeping Percentage: {walk_pert}%"}
//             h3 {"Sleeping Percentage: {exer_pert}%"}
//             h3 {"Sleeping Percentage: {hobby_pert}%"}
//             h3 {"Sleeping Percentage: {learn_pert}%"}
//             h3 {"Sleeping Percentage: {work_self_pert}%"}
//             h3 {"Sleeping Percentage: {work_man_pert}%"}
//         }
//     }
// }

#[component]
fn Home() -> Element {

    let mut counter = Counter::new();

    let mut percent_total = Counter::new();

    // percent_total = Counter::daily_percent(percent_total, &counter);

    rsx! {
        
        div {
            h1 { "Daily Work Life Calculator" }
            // button { onclick: move |_| percent_total = calculate_percent(&counter), "Calculate" }
            h3 { "Sleeping: {counter.sleep}" }
            button { onclick: move |_| counter.sleep += 1.0, "Up high!" }
            button { onclick: move |_| counter.sleep -= 1.0, "Down low!" }
            h3 { "Stupid Mentalcounter. Health Walk: {counter.walk}" }
            button { onclick: move |_| counter.walk += 1.0, "Up high!" }
            button { onclick: move |_| counter.walk -= 1.0, "Down low!" }
            h3 { "Exercise: {counter.exer}" }
            button { onclick: move |_| counter.exer += 1.0, "Up high!" }
            button { onclick: move |_| counter.exer -= 1.0, "Down low!" }
            h3 { "Hobby: {counter.hobby}" }
            button { onclick: move |_| counter.hobby += 1.0, "Up high!" }
            button { onclick: move |_| counter.hobby -= 1.0, "Down low!" }
            h3 { "Learning: {counter.learn}" }
            button { onclick: move |_| counter.learn += 1.0, "Up high!" }
            button { onclick: move |_| counter.learn -= 1.0, "Down low!" }
            h3 { "Working for Self: {counter.work_self}" }
            button { onclick: move |_| counter.work_self += 1.0, "Up high!" }
            button { onclick: move |_| counter.work_self -= 1.0, "Down low!" }
            h3 { "Working for the Man: {counter.work_man}" }
            button { onclick: move |_| counter.work_man += 1.0, "Up high!" }
            button { onclick: move |_| counter.work_man -= 1.0, "Down low!" }
            p {}
            p {}
            h2 { "Balance Percentage" }
            h3 {"Sleeping Percentage: {percent_total.sleep}%"}
            h3 {"Walking Percentage: {percent_total.walk}%"}
            h3 {"Exercise Percentage: {percent_total.exer}%"}
            h3 {"Hobby Percentage: {percent_total.hobby}%"}
            h3 {"Learning Percentage: {percent_total.learn}%"}
            h3 {"Work for Self Percentage: {percent_total.work_self}%"}
            h3 {"Work for the Man Percentage: {percent_total.work_man}%"}
            // button { onclick: move |_| percent_total = calculate_percent(&counter), "Calculate" }
            // button { onclick: move |_| percent_total.daily_percent(&counter), "Calculate"}
            // button { onclick: move |_| percent_total = counter.daily_percent2(), "Calculate"}
        //     //     (percent_total, &counter), "Calculate" }
        // button { onclick: move |_| 
        //     percent_total = (counter.sleep / 24.0)
        //     (percent_total.walk = counter.walk / HOURSDAY)
        //     (percent_total.exer = counter.exer / HOURSDAY)
        //     (percent_total.hobby = counter.hobby / HOURSDAY)
        //     (percent_total.learn = counter.learn / HOURSDAY)
        //     (percent_total.work_self = counter.work_self / HOURSDAY)
        //     (percent_total.work_man = counter.work_man / HOURSDAY), "Calculte"}
                button { onclick: move |_| percent_total.sleep = counter.sleep, "Calculate"}
        }
    }
}

// #[component]
// fn calculate_percent<'a>(counter: &'a Counter) -> Counter {
//     let mut percent: Counter = Counter::new();
//     percent.sleep =  use_signal(|| counter.sleep / HOURSDAY);
//     percent.walk = use_signal(|| counter.walk / HOURSDAY);
//     percent.exer = use_signal(|| counter.exer / HOURSDAY);
//     percent.hobby = use_signal(|| counter.hobby / HOURSDAY);
//     percent.learn = use_signal(|| counter.learn / HOURSDAY);
//     percent.work_self = use_signal(|| counter.work_self / HOURSDAY);
//     percent.work_man = use_signal(|| counter.work_man / HOURSDAY);
//     percent
// }