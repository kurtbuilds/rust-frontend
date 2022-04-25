#![allow(non_snake_case)]

use std::ops::Deref;

fn double(x: u32) -> u32 {
    let doubler = |z| z * z;
    doubler(x)
}

struct EventTarget {
    pub value: string,
}

struct Event {
    pub target: EventTarget
}

fn CounterApp() -> Element {
    let value = use_state(0);

    let foobar = use_effect(|| {

    }, )

    let onclick = |e| *value = e.target.value;
    html! {
        <div class="flex flex-column">
            <div>
                Counter value: {value}
            </div>
            <button onclick={|e| *value = e.target.value.parse::<u32>}>+1 with inline callback</button>
            <button onclick={onclick}>+1 with named callback</button>



        </div>
    }
}

struct BudgetProps {
    count: u32,
}

struct Props<T> {
    children: u32,
    user_props: T,
}

impl<T> Deref for Props<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.user_props
    }
}

// fn Budget(props: Props<BudgetProps>) {
//     props.date
//
//     let budget =
//     html! {
//         <div>
//         </div>
//     }
// }

fn main() {
    let z: Props<BudgetProps> = Props {
        children: 4,
        user_props: BudgetProps {
            count: 12,
        },
    };
    println!("{}", z.count);
    println!("{}", double(z.children));
    println!("Hello, world!");
}
