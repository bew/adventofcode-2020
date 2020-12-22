use std::fs::File;
use std::io::{BufReader, BufRead};

// Prefered signature, using a flexible AsRef for the path type.
// I don't use it (yet) because I don't fully understand AsRef for now.
// pub fn read_lines<P: AsRef<Path>>(filename: P) -> MyResult<Vec<String>> {
pub fn read_lines(filename: &str) -> MyResult<Vec<String>> {
    let file = File::open(filename)
        .with_context(|| format!("Unable to open file {}", filename))?;
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
        lines.push(line.with_context(|| format!("Unable to read line from file {}", filename))?);
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

// The trait must be Sized (it is not by default in a trait declaration),
// because one of the trait method has a default implementation, and the compiler
// needs to know if `Self` (the type for `self`) is Sized so I can pass it as an argument.
// Ref: https://stackoverflow.com/a/30941589/5655255 near the end (great answer!)
pub trait ErrWithContext<T>: Sized {
    // Initial signature:
    //     fn context(self, message: &str) -> Result<T, E>;
    // This does not work in practice, because the message is either &str or String
    // (for example when using `format!("some {} text", "more")`

    // NOTE: We grab `self` and consume it (as the ok value will be moved)
    fn context<M: std::fmt::Display>(self, message: M) -> MyResult<T> {
        self.with_context(|| message)
    }

    // Here is another way to add context to an abj. This function takes an arg-less function
    // so that the caller can allocate memory only when there is an error for example.
    //
    // This form ALWAYS allocates memory for `format!()` call:
    //   Err("invalid").context(format!("foo {}", "bar")
    // This form ONLY allocates memory for `format!()` if needed:
    //   Ok("valid").with_context(|| format!("foo {}", "bar")     -- NO allocation for ok
    //   Err("invalid").with_context(|| format!("foo {}", "bar")  -- allocation for error
    fn with_context<M: std::fmt::Display, F: FnOnce() -> M>(self, message_fn: F) -> MyResult<T>;
}

// Here we implement that trait on Result. The return type is MyResult.
impl<T, E: std::fmt::Display> ErrWithContext<T> for Result<T, E> {
    fn with_context<M: std::fmt::Display, F: FnOnce() -> M>(self, message_fn: F) -> MyResult<T> {
        self.map_err(|err| {
            format!("{}: {}", (message_fn)(), err)
        })
    }
}

// Here we implement that trait on Option. The return type is MyResult.
impl<T> ErrWithContext<T> for Option<T> {
    fn with_context<M: std::fmt::Display, F: FnOnce() -> M>(self, message_fn: F) -> MyResult<T> {
        // The impl is pretty simple, since `Option` already provides a method to get
        // a Result obj.
        //
        // NOTE: .to_string comes from std::string::ToString trait, and is implemented
        //       for any type that implements std::fmt::Display.
        self.ok_or_else(|| (message_fn)().to_string())
    }
}
