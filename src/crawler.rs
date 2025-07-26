use crate::parser::extract_links;
use anyhow::Result;
use reqwest::Client;
use std::collections::{HashSet, VecDeque};
use tokio::sync::Semaphore;
use url::Url;

pub struct Crawler {
    client: Client,
    visited: HashSet<Url>,
    queue: VecDeque<Url>,
    sem: Semaphore,
}

impl Crawler {
    pub fn new(start: Url, concurrency: usize) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        Self {
            client: Client::new(),
            visited: HashSet::new(),
            queue,
            sem: Semaphore::new(concurrency),
        }
    }

    pub async fn run(&mut self, limit: usize) -> Result<()> {
        while let Some(url) = self.queue.pop_front() {
            if self.visited.len() >= limit {
                break;
            }
            if !self.visited.insert(url.clone()) {
                continue; // already seen
            }

            // Acquire a permit for concurrency
            let permit = self.sem.acquire().await?;
            let client = self.client.clone();

            // Spawn a task per URL
            tokio::spawn(async move {
                let _permit = permit; // keep alive until task completes
                if let Ok(body) = client.get(url.clone()).send().await.and_then(|r| r.text().await) {
                    for link in extract_links(&body, &url) {
                        // In real code, you'd send link back to the main queue
                        println!("Found: {}", link);
                    }
                }
            });
        }

        // wait for all tasks to finish
        // (dropping the semaphore permits when tasks end)
        Ok(())
    }
}
