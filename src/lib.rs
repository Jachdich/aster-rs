
pub struct Client {
    on_packet: fn(&mut Client, &serde_json::Value),
}

impl Client {
    pub fn handle_packet(&mut self, packet: serde_json::Value) {
        // let on_packet = self.on_packet;
        std::thread::spawn(move || {
            (self.on_packet)(self, &packet);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    fn on_packet(client: &mut Client, packet: &serde_json::Value) {
        println!("{:?}", packet);
    }

    #[test]
    fn main() {
        let mut c = Client { on_packet };
        c.handle_packet(json!({"a": "b"}));
    }
}