use chrono::Local;

pub fn get_time_string() -> String {
    Local::now().format("%H:%M:%S%.3f").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}:プログラム開始", get_time_string());
    }
}
