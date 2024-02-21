macro_rules! make_id_type {
    ($IdType: ty) => {
        paste::paste! {
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub struct [<$IdType Godot>](pub $IdType);

            impl GodotConvert for [<$IdType Godot>] {
                type Via = u32;
            }

            impl ToGodot for [<$IdType Godot>] {
                fn to_godot(&self) -> Self::Via {
                    self.0 .0
                }
            }

            impl FromGodot for [<$IdType Godot>] {
                fn try_from_godot(via: Self::Via) -> Option<Self> {
                    Some([<$IdType Godot>]($IdType(via)))
                }
            }

            impl std::convert::From<$IdType> for [<$IdType Godot>] {
                fn from(value: $IdType) -> Self {
                    [<$IdType Godot>](value)
                }
            }
        }
    };
}
