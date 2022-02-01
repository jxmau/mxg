use crate::{kind::Kind, Message};

/// Info Struct used to pass informations when not debugging.
/// This struct cannot be added to the Buffer yet. 
/// To consume it, call the method consume or just let it be dropped.
/// 
/// Please note that this struct, and this feature is experimental, 
/// and can be only used outside the regular workflow of the library
pub struct InfoMessage {
    kind: Kind,
    file: String,
    title: String,
    explain: String,
}

impl InfoMessage {

    /// Create a new InfoMessage struct
    pub fn new(kind: Kind, file: String, title: String, explain: String) -> Self {
        Self {kind, file, title, explain}
    }

}



impl Message for InfoMessage {
    
    fn consume(&self) {
        use ansi_term::Colour::{Red, Green, White, Yellow};
        use Kind::*;
        let prefix: String = match &self.kind {
            Error => Red.paint("CRITICAL INFO:"),
            Warning => Yellow.paint("WARNING INFO"),
            Valid => Green.paint("INFO"),
            _ => White.paint("INFO"),
        }.to_string();
        println!("{prefix} for {} | {}\n\n    {}  \n", self.file, self.title, self.explain);
    }

    fn kind(&self) -> Kind {
        self.kind.clone()
    }
}

#[cfg(feature= "buffless")]
impl Drop for InfoMessage {
    fn drop(&mut self) {
        self.consume()
    }
}

impl super::Boxable for InfoMessage {}