// use actix::{Actor, Addr, Context, System};

// // In order to define an actor you need to define a struct and have it implement the Actor trait.
// struct MyActor;

// // implementing an Actor
// impl Actor for MyActor {
//     type Context=Context<Self>;

//     ///
//     /// Spawning a new actor is achieved via the start and create methods of the Actor trait. It provides several different ways of 
//     /// creating actors; for details, check the docs. You can implement the started, stopping and stopped methods of the Actor trait.
//     /// started gets called when the actor starts and stopping when the actor finishes. Check the API docs for more information on 
//     /// the actor lifecycle.
//     /// 
//     fn started(&mut self,ctx: &mut Self::Context) {
//         println!("I am alive!");
//         System::current().stop();  // <- stop system
//     }
// }


// fn main() {
//     // In order to use actix you first need to create a System
//     let system = actix::System::new();

//     // Actix uses the Tokio runtime. System::new() creates a new event loop. System.run() starts the Tokio event loop,
//     // and will finish once the System actor receives the SystemExit message.

//     let addr = system.block_on(async { MyActor.start()});
//     system.run();
// }


// --------------------------- Handler
///  An Actor communicates with another Actor by sending messages. In actix all messages are typed. Let's define a simple 
/// Sum message with two usize parameters and an actor which will accept this message and return the sum of those two numbers. 
///
// use actix::prelude::*;

// // this is our Message
// // we have to define the response type (rtype)
// #[derive(Message)]
// #[rtype(result="usize")]
// struct Sum(usize,usize);

// // Actor definition
// struct Calculator;

// impl Actor for  Calculator {
//     type Context=Context<Self>;
// }

// // now we need to implement `Handler` on `Calculator` for the `Sum` message.
// impl Handler<Sum> for Calculator {
//     type Result = usize;  // <- Message response type

//     fn handle(&mut self, msg: Sum, ctx: &mut Context<Self>) -> Self::Result {
//         msg.0 + msg.1
//     }
// }

// #[actix::main]   // <- starts the system and block until future resolves
// async fn main() {
//     let addr = Calculator.start();  // All communications with actors go through an Addr object. You can do_send a message without waiting for a response
//     let res = addr.send(Sum(10,5)).await; // <- send message and get future for result

//     match res {
//         Ok(result) => println!("Sum: {}",result),
//         _ => println!("Communication to the actor has failed")
//     }
// }


use actix::prelude::*;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
struct Ping {
    pub id: usize,
}

// Actor definition
struct Game {
    counter: usize,
    name: String,
    recipient: Recipient<Ping>,
}

impl Actor for Game {
    type Context = Context<Game>;
}

// simple message handler for Ping message
impl Handler<Ping> for Game {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1; // 每个人接受十次

        if self.counter > 10 {
            System::current().stop();
        } else {
            println!("[{0}] Ping received {1}，counter {2}", self.name, msg.id,self.counter);

            // wait 100 nanoseconds
            ctx.run_later(Duration::new(0, 100), move |act, _| {
                act.recipient.do_send(Ping { id: msg.id + 1 });
            });
        }
    }
}

fn main() {
    let mut system = System::new();

    // To get a Recipient object, we need to use a different builder method
    // which will allow postponing actor creation
    let addr = system.block_on(async {
        Game::create(|ctx| {
            // now we can get an address of the first actor and create the second actor
            let addr = ctx.address();

            let addr2 = Game {
                counter: 0,
                name: String::from("Game 2"),
                recipient: addr.recipient(),  // addr2的接受者是addr
            }
            .start();

            // let's start pings
            addr2.do_send(Ping { id: 10 });  // 给addr发消息


            let addr3 = Game {
                counter: 0,
                name: String::from("Game 3"),
                recipient: addr2.recipient(),  // addr3的接受者是addr2
            }
            .start();

            // let's start pings
            addr3.do_send(Ping { id: 10 });  // 给addr2发消息


            // now we can finally create first actor
            Game {
                counter: 0,
                name: String::from("Game 1"),
                recipient: addr3.recipient(),   // addr的接受者又是addr3
            }
        });
    });

    system.run();
}
