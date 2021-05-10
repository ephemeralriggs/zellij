use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, StructOpt)]
/// Options that can be set either through the config file,
/// or cli flags
pub struct Options {
    /// Allow plugins to use a more compatible font type
    #[structopt(long)]
    pub simplified_ui: bool,
}

impl Options {
    pub fn from_yaml(from_yaml: Option<Options>) -> Options {
        if let Some(opts) = from_yaml {
            opts
        } else {
            Options::default()
        }
    }
}
