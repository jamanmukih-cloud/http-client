pub struct HttpClient {
    pool: ConnectionPool,
    retry: RetryPolicy,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            pool: ConnectionPool::new(),
            retry: RetryPolicy::default(),
        }
    }
    
    pub async fn get(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let conn = self.pool.get(url)?;
        Ok(Response::new(200))
    }
}

pub struct Response { status: u16 }
impl Response {
    fn new(status: u16) -> Self { Self { status } }
    pub fn status(&self) -> u16 { self.status }
}

pub struct ConnectionPool;
impl ConnectionPool {
    fn new() -> Self { Self }
    fn get(&self, _url: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
}

pub struct RetryPolicy { max_retries: u32 }
impl Default for RetryPolicy {
    fn default() -> Self { Self { max_retries: 3 } }
}
