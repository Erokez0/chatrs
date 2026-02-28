pub struct NounMap;
impl NounMap {
    pub fn get(key: u8) -> String {
        match key {
            0 => "beaver".to_string(),
            1 => "one".to_string(),
            2 => "guy".to_string(),
            3 => "user".to_string(),
            4 => "butterfly".to_string(),
            5 => "tea pot".to_string(),
            6 => "giraffe".to_string(),
            7 => "guest".to_string(),
            8 => "corpse".to_string(),
            9 | _ => "fan".to_string(),
        }
    }
}
