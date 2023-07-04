// use std::fs;

async fn get_logs() -> &'static str {

    //let contents = fs::read_to_string("./src/error.log");
    let contents = include_str!("./error.log");

    contents
}

#[tokio::main]
async fn main() {
    let data = get_logs().await;
    println!("{}", data);
}

#[cfg(test)]
mod tests {
    use super::get_logs;

    #[tokio::test]
    async fn test_logs() {
        let data = get_logs().await;
        assert_eq!(data, "logs: 1000");
    }
}