//! Windows Clipboard dummy implementation
//!

use std::string::FromUtf8Error;

use super::{Load, Store};
pub struct Clipboard;

#[derive(Debug)]
pub enum Error {
    Utf8(FromUtf8Error),
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Utf8(ref err) => Some(err),
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::Utf8(..) => "clipboard contents not utf8",
        }
    }
}


impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::Utf8(ref err) => write!(f, "error parsing utf8 string: {}", err),
        }
    }
}


impl Load for Clipboard {
    type Err = Error;

    fn new() -> Result<Self, Error> {
        Ok(Clipboard)
    }

    fn load_primary(&self) -> Result<String, Self::Err> {
        Ok(String::with_capacity(0))
    }

    fn load_selection(&self) -> Result<String, Self::Err> {
        Ok(String::with_capacity(0))
    }
}

impl Store for Clipboard {
    /// Sets the primary clipboard contents
    #[inline]
    fn store_primary<S>(&mut self, contents: S) -> Result<(), Self::Err>
        where S: Into<String>
    {
        Ok(())
    }

    /// Sets the secondary clipboard contents
    #[inline]
    fn store_selection<S>(&mut self, contents: S) -> Result<(), Self::Err>
        where S: Into<String>
    {
        Ok(())
    }
}

impl Clipboard {
}

#[cfg(test)]
mod tests {
    use super::Clipboard;
    use ::{Load, Store};

    #[test]
    fn create_clipboard_save_load_contents() {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.store_primary("arst").unwrap();
        let loaded = clipboard.load_primary().unwrap();
        assert_eq!("arst", loaded);
    }
}
