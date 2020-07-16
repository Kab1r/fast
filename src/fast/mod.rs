mod models;
mod scraper;
use async_stream::stream;
use async_stream::try_stream;
use futures::pin_mut;
use futures::stream::Stream;
use futures_util::stream::StreamExt;
use models::{FastJson, FastTarget};
use reqwest::{self, blocking::Client, Error};
use std::sync::{
    atomic::{AtomicU16, AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;
use tokio::{
    task::{self, JoinHandle},
    time::{self, Duration},
};

const USER_AGENT: &str = "github.com/kabirkwatra/fast";
const UPDATE_DELAY: Duration = Duration::from_secs(1);

pub struct Fast {
    token: String,
    api_url: String,
    num_endpoints: u16,
    pub max_payload_length: usize,
    client: Client,
}

impl Fast {
    pub fn new() -> Fast {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("HTTP client failed to build");

        let javascript = scraper::get_js_file(&client).unwrap();
        Fast {
            client,
            token: scraper::get_token(&javascript),
            num_endpoints: scraper::get_url_count(&javascript),
            max_payload_length: scraper::get_max_payload_length(&javascript),
            api_url: scraper::get_api_endpoint(&javascript),
        }
    }

    /// Returns a vector of urls
    pub fn get_urls(&self) -> Result<Vec<String>, Error> {
        let res = self
            .client
            .get(self.api_url.as_str())
            .query(&[
                ("https", "true"),
                ("token", self.token.as_str()),
                ("urlCount", self.num_endpoints.to_string().as_str()),
            ])
            .send()?;

        let data = res.json::<FastJson>()?;
        let targets = data
            .targets
            .iter()
            .map(|target: &FastTarget| target.url.clone())
            .collect::<Vec<_>>();
        Ok(targets)
    }

    /// Returns a stream of the total number of bytes downloaded
    ///
    /// * Returns `None` when downloads are complete
    pub fn measure(
        urls: Vec<String>,
        n: usize,
        max_payload_length: usize,
    ) -> impl Stream<Item = Option<f64>> {
        stream! {
            let byte_length_totals = stream! {
                let total = Arc::new(AtomicUsize::new(0));
                let completed = Arc::new(AtomicU16::new(0));
                let mut tasks: Vec<JoinHandle<()>> = vec![];

                for url in &urls {
                    let mut payload_length = max_payload_length;
                    for _ in 0..n {
                        let total = Arc::clone(&total);
                        let completed = Arc::clone(&completed);
                        let url = Fast::insert_length(url, payload_length);
                        tasks.push(Fast::create_task(url, total, completed));
                        payload_length -= 1;
                    }
                }
                // Until all tasks complete
                while &(completed.load(Ordering::Relaxed) as usize) < &tasks.len() {
                    time::delay_for(UPDATE_DELAY).await;
                    let total = Arc::clone(&total);
                    yield Some(total.load(Ordering::Relaxed));
                }
                yield None;
            };

            pin_mut!(byte_length_totals);
            let time_at_first_request = Instant::now();

            while let Some(length) = byte_length_totals.next().await {
                match length {
                    None => {yield None; break;},
                    Some(length) => {
                        let time_diff = time_at_first_request.elapsed().as_secs_f64();
                        let kbps = (length as f64 / time_diff * 8f64 / 1000f64);
                        yield Some(kbps);
                    }
                }
            }
        }
    }
    /// Returns a `JoinHandler` for the task created
    fn create_task(
        url: String,
        total: Arc<AtomicUsize>,
        completed: Arc<AtomicU16>,
    ) -> JoinHandle<()> {
        task::spawn(async move {
            let downloads = Fast::download(url.clone());
            pin_mut!(downloads);
            while let Some(length) = downloads.next().await {
                total.fetch_add(length.unwrap(), Ordering::Relaxed);
            }
            completed.fetch_add(1, Ordering::Relaxed);
        })
    }
    /// Returns a stream of the number of bytes downloaded
    fn download(url: String) -> impl Stream<Item = Result<usize, Error>> {
        try_stream! {
            let mut res = reqwest::get(url.as_str()).await?;
            while let Some(item) = res.chunk().await? {
                yield item.len();
            }
        }
    }
    /// Retruns a url with the given payload length inserted as a range
    fn insert_length(url: &str, payload_length: usize) -> String {
        url.replace(
            "/speedtest",
            format!("{}/0-{}", "/speedtest/range", payload_length).as_str(),
        )
    }
}
