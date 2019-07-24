/* Allows for coercion of rusqlite::Error into UserError */

// Third Party Dependencies
use scrawl::ScrawlError;

// Intra Library Imports
use super::UserError;

/// Convert a ScrawlError into a UserError
///
/// # Example
/// ```should_panic
/// use scrawl;
/// 
/// fn bad_scrawl() -> Result<String, UserError> {
///     scrawl::open("does_not_exist.txt")
/// }
///
/// match bad_scrawl() {
///     Ok(s)  => println!("{}", s),
///     Err(e) => e.print_and_exit()
/// }
/// 
/// ```
impl From<ScrawlError> for UserError {
    fn from(error: ScrawlError) -> Self {
        const SUMMARY: &str = "Scrawl Error";
        match self {
            ScrawlError::EditorNotFound => UserError::hardcoded(SUMMARY,
                    &["Could not determine the user's preferred editor"],
                    &["Make sure your $EDITOR environment variable is set."]),

            ScrawlError::FailedToCreateTempfile => UserError::hardcoded(SUMMARY,
                    &["Could not create a temporary file to use as a buffer"],
                    &[]),

            ScrawlError::FailedToOpenEditor(editor) => UserError::hardcoded(SUMMARY,
                    &[&format!("Could not open {} as a text editor", editor),
                    &[]),

            ScrawlError::FailedToReadIntoString => UserError::hardcoded(SUMMARY,
                    &["Failed to parse file into valid UTF-8 String."],
                    &[]),

            ScrawlError::FailedToCopyToTempFile(filename) => UserError::hardcoded(SUMMARY,
                &[&format!("Failed to copy the contents of the `{}` to the temporary buffer for editing.", filename)],
                &["Make sure the file exists."]),


            ScrawlError::Other(string) => UserError::simple(string)
        }
    }
}
