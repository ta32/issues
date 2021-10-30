use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::{Result};

pub fn get_dai_price() -> Result<Response> {
    let client = Client::new();
    let resp = client.get("https://api.coingecko.com/api/v3/coins/dai/history?date=30-12-2019&localization=false").send()?;
    return Ok(resp);
}

fn main() {
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert!(true);
    }

}