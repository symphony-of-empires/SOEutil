use super::release::Releases;
use log::{error, info};
use reqwest::header::USER_AGENT;
use reqwest::Client;

pub async fn fetch_releases(url: &str) -> Result<Releases, ()> {
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.79 Safari/537.36";

    info!("Downloading latest release from {url}");
    info!("User agent set to {user_agent}");

    let client = Client::new();
    let request = client.get(url).header(USER_AGENT, user_agent);

    let response = match request.send().await {
        Ok(response) => response,
        Err(error) => {
            error!("Error while sending the fetch request to {url}");
            info!("Message {:?}", error);
            return Err(());
        }
    };

    let releases = match response.json::<Releases>().await {
        Ok(releases) => releases,
        Err(error) => {
            error!("Error while deserializing the response json");
            info!("Message {:?}", error);
            return Err(());
        }
    };

    Ok(releases)
}
