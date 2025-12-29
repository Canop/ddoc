use {
    crate::*,
    std::{
        fs,
        path::PathBuf,
    },
    termimad::crossterm::style::Stylize,
};

pub struct Page {
    pub title: String,
    pub page_path: PagePath,
    pub md_file_path: PathBuf,
}

impl Page {
    pub fn new(
        title: String,
        page_path: PagePath,
        md_file_path: PathBuf,
    ) -> Self {
        Self {
            title,
            page_path,
            md_file_path,
        }
    }

    /// Write the full HTML for this page into the given `html` String
    ///
    /// # Errors
    /// Return `DdError` variants on write errors, not on project config/data errors
    pub fn write_html(
        &self,
        html: &mut String,
        project: &Project,
    ) -> DdResult<()> {
        let Ok(md) = fs::read_to_string(&self.md_file_path) else {
            eprintln!(
                "{} {} could not be read, skipping.",
                "ERROR:".red().bold(),
                self.md_file_path.to_string_lossy().yellow()
            );
            return Ok(());
        };
        let page_writer = PageWriter::new(self, project, md)?;
        page_writer.write_html(html)
    }
}
