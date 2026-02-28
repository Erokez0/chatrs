pub struct AdjectiveMap;
impl AdjectiveMap {
    pub fn get(key: u8) -> String {
        match key {
            0 => "cool".to_string(),
            1 => "violent".to_string(),
            2 => "musical".to_string(),
            3 => "illegal".to_string(),
            4 => "purple".to_string(),
            5 => "fake".to_string(),
            6 => "unwanted".to_string(),
            7 => "cold".to_string(),
            8 => "genius".to_string(),
            9 | _ => "lazy".to_string(),
        }
    }
}
