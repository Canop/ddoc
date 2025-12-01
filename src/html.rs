pub static HTML_START: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1"/>
"#;

pub fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('"', "&quot;")
}
pub fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
