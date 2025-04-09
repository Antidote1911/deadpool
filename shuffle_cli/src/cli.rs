use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author,
    about,
    version,
    after_help = "If you do not specify any of the \
[--uppercase, --lowercase, --digits] flags, \
then lowercase and digits will be used."
)]
pub struct Cli {
    /// Use UPPERCASE letters [A-Z]
    #[clap(short, long)]
    pub(crate) uppercase: bool,

    /// Use lowercase letters [a-z]
    #[clap(short, long)]
    pub(crate) lowercase: bool,

    /// Use digits [0-9]
    #[clap(short, long)]
    pub(crate) digits: bool,

    /// Use special symbols [*&^%$#@!~]
    #[clap(short, long)]
    pub(crate) braces: bool,

    #[clap(short, long)]
    pub(crate) punctuation: bool,

    #[clap(short, long)]
    pub(crate) quotes: bool,

    #[clap(long)]
    pub(crate) dashes: bool,

    #[clap(short, long)]
    pub(crate) math: bool,

    #[clap(long)]
    pub(crate) logograms: bool,

    /// Number of passwords to generate
    #[clap(short = 'C', long, value_name = "NUMBER", default_value = "1")]
    count: usize,

    /// Sets the required password length
    #[clap(short = 'L', long, value_name = "NUMBER", default_value = "10")]
    length: usize,

    /// Output in a txt file
    #[clap(long)]
    output: Option<String>,

    /// Exclude char
    #[clap(long)]
    exclude: Option<String>,

    /// include char
    #[clap(long)]
    include: Option<String>,
}

impl Cli {
    pub fn validate(&mut self) -> Result<(), String> {
        if !self.uppercase && !self.lowercase && !self.digits && !self.braces && !self.punctuation && !self.quotes && !self.dashes && !self.math && !self.logograms{
        }

        Ok(())
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn output(&self) -> Option<String> {
        self.output.clone()
    }

    pub fn exclude(&self) -> Option<String> {
        self.exclude.clone()
    }

    pub fn include(&self) -> Option<String> {
        self.include.clone()
    }
}