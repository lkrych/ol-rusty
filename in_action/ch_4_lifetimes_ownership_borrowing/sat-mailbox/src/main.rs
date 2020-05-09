#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) { //&self indicates that GroundStation.send() only requires a read-only reference to self
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}


fn main() {
   let base = GroundStation {};
   let mut sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] }};

   println!("t0: {:?}", sat_a);
   base.send(&mut sat_a, Message::from("hello there!")); // We don't have an ergonomic way to make messages yet, so we will use the String function from

   println!("t1: {:?}", sat_a);

   let msg = sat_a.recv();
   println!("t2: {:?}", sat_a);
   println!("msg: {:?}", msg);
}
