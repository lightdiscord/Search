use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Engine {
    pub name: String,
    pub schema: String
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
    ( $( $id:ident($name:expr, $schema:expr) ),* ) => {
        use super::Engine;

        pub enum Engines {
            $($id),*
        }

        impl Into<Engine> for Engines {
            fn into(self) -> Engine {
                match self {
                    $(
                        Engines::$id => Engine {
                            name: $name.into(),
                            schema: $schema.into()
                        }
                    ),*
                }
            }
        }

        impl Engines {
            pub fn to_vec() -> Vec<Engine> {
                vec![$(Engines::$id.into()),*]
            }
        }
    };
}

pub mod list;
