#![allow(unused_variables, dead_code)]
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation{}

impl MailBox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg)
            }
        }

        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat{
            id: sat_id,
        }
    }

    fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = MailBox{messages: vec![]};
    let base = GroundStation{};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message{to: sat_id, content: String::from("hello")};
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
