# concurrent-web-crawler

A high‑performance, concurrent web crawler in Rust. Starting from a seed URL (or list), it obeys robots.txt, fetches pages in parallel, extracts links, and builds a site‑map in JSON or CSV—all while respecting rate limits and domain constraints.

### Features
#### Concurrent Fetching
- Uses tokio and async/await to schedule hundreds of HTTP requests in flight
- Adjustable concurrency limit (--workers) to tune throughput versus politeness

#### Robots.txt Compliance
- Automatically discovers and parses each domain’s `robots.txt`
- Honors crawl‑delay and disallows rules before fetching any URLs

#### URL Extraction & Filtering
- Parses HTML with `select.rs` (or scraper) to extract <a href="…"> links
- Normalizes URLs, resolves relative paths, and deduplicates
- Domain‑ and path‑based inclusion/exclusion filters

#### Rate Limiting & Politeness
- Per‑domain rate limiter using `tokio::sync::Semaphore` and timers
- Global rate cap option to avoid overloading hosts

#### Depth‑First or Breadth‑First Traversal
- Configurable crawl strategy (`--strategy bfs|dfs`)
- Maximum depth limit to bound discovery

### Installation
```
# Clone and build
git clone https://github.com/your-username/concurrent-web-crawler.git
cd concurrent-web-crawler
cargo build --release

# (Optional) Install as CLI tool
cargo install --path .
```

### Usage
```
# Crawl a single seed URL to depth 3, 20 workers, 1 request/sec per domain
concurrent-web-crawler \
  --seeds https://example.com \
  --depth 3 \
  --workers 20 \
  --rate 1 \
  --output sitemap.json
```

```
# Crawl from a list of seeds, BFS strategy, exclude `/private` paths, CSV output
concurrent-web-crawler \
  --seeds-file seeds.txt \
  --strategy bfs \
  --exclude-path '/private' \
  --output sitemap.csv
```

### License
This project is licensed under the MIT License. Contributions and issue reports are very welcome!
