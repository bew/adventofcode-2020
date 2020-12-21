use std::fs::File;
use std::io::{BufReader, BufRead};

// Prefered signature, using a flexible AsRef for the path type.
// I don't use it (yet) because I don't fully understand AsRef for now.
// pub fn read_lines<P: AsRef<Path>>(filename: P) -> MyResult<Vec<String>> {
pub fn read_lines(filename: &str) -> MyResult<Vec<String>> {
    let file = File::open(filename)
        .context(format!("Unable to open file {}", filename))?;
    let reader = BufReader::new(file);

    // Initial impl:
    // Ok(reader.lines()
    //     .map(|line| { line.expect("Unable to read line") })
    //     .collect())
    // BUT this uses `line.expect` and does not handle the error.
    // In that form we CANNOT use `line?` because we're inside a closure,
    // and that closure is not supposed to return a Result obj.
    //
    // SO we rewrite it in a non-closure way:
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.context(format!("Unable to read line from file {}", filename))?);
    }
    Ok(lines)
}

// ---------------------
// Basic error handling helper, for better libs in the future, checkout:
// https://nick.groenen.me/posts/rust-error-handling/
// https://rust-cli.github.io/book/tutorial/errors.html

// This is a custom Result alias, that has the error type always be a String,
// as returned by ErrWithContext::context method.
pub type MyResult<T> = Result<T, String>;

pub trait ErrWithContext<T> {
    // NOTE: We grab `self` and consume it (as the ok value will be moved)
    fn context<M: std::fmt::Display>(self, message: M) -> MyResult<T>;

    // Initial signature:
    //     fn context(self, message: &str) -> Result<T, E>;
    // This does not work in practice, because the message is either &str or String
    // (for example when using `format!("some {} text", "more")`
}

// Here we implement that trait on Result. The return type is MyResult.
impl<T, E: std::fmt::Display> ErrWithContext<T> for Result<T, E> {
    fn context<M: std::fmt::Display>(self, message: M) -> MyResult<T> {
        self.map_err(|err| {
            format!("{}: {}", message, err)
        })
    }
}

// Here we implement that trait on Option. The return type is MyResult.
impl<T> ErrWithContext<T> for Option<T> {
    fn context<M: std::fmt::Display>(self, message: M) -> MyResult<T> {
        // The impl is pretty simple, since `Option` already provides a method to get
        // a Result obj.
        self.ok_or_else(|| message.to_string())
    }
}
