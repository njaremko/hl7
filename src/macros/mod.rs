#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}
