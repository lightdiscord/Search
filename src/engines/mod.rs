#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Engine {
    pub name: &'static str,
    pub schema: &'static str
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            name: "".into(),
            schema: "".into(),
        }
    }
}

macro_rules! engines {
    ( $( $id:ident => ($name:expr, $schema:expr) ),* ) => {
        use super::Engine;

        $(
            pub const $id: Engine = Engine {
                name: $name,
                schema: $schema,
            };
        )*

        pub const ENGINES: &'static [Engine] = &[$($id),*];
    };
}

pub mod list;
