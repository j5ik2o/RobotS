#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate robots;

use robots::{Actor, ActorSystem, CanReceive, Message, Props};

struct MyActor;

impl Actor for MyActor {
    fn receive(&self, message: Message) {
        match message {
            Message::Text(s) => println!("I received a text message: ({}) !", s),
            Message::Dummy => println!("I received a dummy message !"),
        }
    }
}

impl MyActor {
    fn new(_dummy: ()) -> MyActor {
        MyActor
    }
}

fn main() {

    let actor_system = ActorSystem::new("test".to_owned());
    actor_system.spawn_threads(5);

    let props_1 = Props::new(Box::new(MyActor::new), ());
    let actor_ref_1 = actor_system.actor_of(props_1);

    let props_2 = Props::new(Box::new(MyActor::new), ());
    let actor_ref_2 = actor_system.actor_of(props_2);

    actor_ref_1.receive(Message::Dummy);
    actor_ref_1.receive(Message::Text("Hello there".to_owned()));
    actor_ref_1.handle();
    actor_ref_1.handle();

    std::thread::sleep_ms(700);
    actor_system.terminate_threads(5);
    std::thread::sleep_ms(700);

    println!("Hello world!");
}
