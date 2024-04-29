use std::default::Default;

struct Device {
    name: String,
    id: u64, 
    parts_needing_repair: Vec<String>,
    desc_damage: String,
    curr_station: u8,
    total_stations: u8,
}

// implements default values so not every value needs to be 
// specified with every instantation
impl Default for Device {
    fn default() -> Device {
        Device{
            name: "{UNIDENTIFIED}".to_string(),
            id: 0,
            parts_needing_repair: vec!["{BLANK}".to_string()],
            desc_damage: "{BLANK}".to_string(),
            curr_station: 0,
            total_stations: 0,
        }
    }
}

fn main() {
    // example device
    let mut iPhone_15 = Device::default();
    iPhone_15.name = "Apple iPhone 15".to_string();
    iPhone_15.id = 1;
    iPhone_15.desc_damage = "- needs new screen".to_string();
    iPhone_15.parts_needing_repair = vec!["screen".to_string(), "screen cable".to_string()]
}
