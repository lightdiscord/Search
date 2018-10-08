pub struct EngineStructure {
    pub name: String,
    pub schema: String,
}

impl Default for EngineStructure {
    fn default() -> Self {
        Self {
            name: "".into(),
            schema: "".into(),
        }
    }
}

macro_rules! engines {
    ( $( $id:ident($name:expr, $schema:expr) ),* ) => {
        use rand::thread_rng;
        use rand::seq::SliceRandom;

        use super::EngineStructure;

        #[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum Engine {
            $($id),*
        }

        impl Engine {
            pub fn to_vec() -> Vec<Engine> {
                vec![$(Engine::$id),*]
            }

            pub fn metas(&self) -> EngineStructure {
                match *self {
                    $(
                        Engine::$id => EngineStructure {
                            name: $name.into(),
                            schema: $schema.into(),
                        }
                    ),*
                }
            }
        }

        impl Default for Engine {
            fn default() -> Self {
                Engine::to_vec().choose(&mut thread_rng()).unwrap().to_owned()
            }
        }
    };
}

mod engines;
pub use self::engines::Engine;
