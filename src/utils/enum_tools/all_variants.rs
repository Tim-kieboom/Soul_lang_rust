#[macro_export]
macro_rules! enum_with_variants {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $variant:ident $(= $val:expr)?
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $variant $(= $val)?
            ),*
        }

        impl $name {
            $vis const fn all_variants() -> &'static [$name] {
                &[
                    $(
                        $name::$variant
                    ),*
                ]
            }
        }
    };
}