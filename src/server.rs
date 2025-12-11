//! nothing yet
use {
    crate::*,
    rouille::Response,
    std::path::PathBuf,
};

pub struct Server {
    base_url: String,
    addr: String,
    static_path: PathBuf,
}

impl Server {
    pub fn new(
        static_path: PathBuf,
        port: u16,
    ) -> DdResult<Self> {
        let addr = format!("localhost:{port}");
        let base_url = format!("http://{addr}/");
        Ok(Self {
            base_url,
            addr,
            static_path,
        })
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn run(self) -> DdResult<()> {
        let static_path = self.static_path;
        let rouille_server = rouille::Server::new(self.addr, move |request| {
            // build the file path
            let mut path = static_path.clone();
            path.push(&request.url()[1..]); // Remove leading /

            if path.is_dir() {
                if request.url().ends_with('/') {
                    // If it's a directory with trailing /,
                    // the URL is correct but we must send index.html
                    path.push("index.html");
                    if path.exists() {
                        if let Ok(file) = std::fs::File::open(&path) {
                            return Response::from_file("text/html", file);
                        }
                    }
                } else {
                    // The URL is missing a trailing /
                    let new_url = format!("{}/", request.url());
                    return Response::redirect_301(new_url);
                }
            }

            // Try to serve the file
            rouille::match_assets(request, &static_path)
        })
        .map_err(|e| DdError::Server(e.to_string()))?;
        rouille_server.run();
        Ok(())
    }
}
