use std::sync::mpsc;
pub struct Broadcast {
    // multiple senders
    channels: Vec<mpsc::Sender<String>>,  
}

impl Broadcast {
    pub fn new() ->Self {
        Broadcast {
            channels: Vec::new(),
        }
    }

    pub fn add_channel(&mut self, channel: mpsc::Sender<String>) {
        self.channels.push(channel);
    }

    pub fn subscribe(&mut self) -> mpsc::Receiver<String>{
        // create a new channel and add the sender
        let (sender, receiver) = mpsc::channel();
        self.add_channel(sender);
        receiver
    }

    pub fn broadcast(&mut self, message: Vec<String>) {
        for channel in self.channels.iter() {
            for msg in message.iter() {
                // send message to each channel
                let result = channel.send(msg.clone());
                if result.is_err() {
                    println!("Error: {:?}", result);
                }
            }
        }
    }

}





