use ffsend_api::action::download::Download as ApiDownload;
use ffsend_api::file::file::DownloadFile;
use ffsend_api::reqwest::Client;

use cmd::cmd_download::CmdDownload;

/// A file download action.
pub struct Download<'a> {
    cmd: &'a CmdDownload<'a>,
}

impl<'a> Download<'a> {
    /// Construct a new download action.
    pub fn new(cmd: &'a CmdDownload<'a>) -> Self {
        Self {
            cmd,
        }
    }

    /// Invoke the download action.
    // TODO: create a trait for this method
    pub fn invoke(&self) {
        // Get the download URL
        let url = self.cmd.url();

        // Create a reqwest client
        let client = Client::new();

        // Parse the file based on the URL
        let file = DownloadFile::parse_url(url)
            .expect("invalid download URL, could not parse file data");

        // Execute an download action
        // TODO: do not unwrap, but return an error
        ApiDownload::new(&file).invoke(&client).unwrap();

        // TODO: open the file, or it's location
        // TODO: copy the file location

        println!("Download complete");
    }
}
