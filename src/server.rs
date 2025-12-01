//! nothing yet
use {
    crate::*,
    rouille::Response,
    termimad::crossterm::style::Stylize,
};

pub fn serve_project(
    project: &Project,
    port: u16,
) -> DdResult<()> {
    let addr = format!("localhost:{port}");
    let static_path = project.build_path.clone();
    eprintln!(
        "Serving {} at {}",
        project.config.title.to_string().yellow().bold(),
        format!("http://{addr}/").green().bold(),
    );
    let server = rouille::Server::new(addr, move |request| {
        // build the file path
        let mut path = static_path.to_owned();
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
    server.run();
    Ok(())
}
