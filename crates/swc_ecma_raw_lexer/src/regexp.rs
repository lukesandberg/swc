use logos::Logos;

use crate::LogosError;

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(error = LogosError)]
enum RegexpContent {
    #[regex(r#"\[[^\]]+\]"#)]
    Class,

    #[regex(r#"\\."#)]
    Escape,

    #[token(r"/")]
    Terminate,

    #[regex(r#"[^\\\[/]+"#)]
    Normal,
}
