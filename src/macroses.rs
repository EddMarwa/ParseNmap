pub type SameType<T> = T;
//Macroses page
// macroses page
// p
// xro
// brain rot]
#[macro_export]
macro_rules! Initializer {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
        #[derive(serde::Deserialize, serde::Serialize, $($derive),*)]
        $pub struct $name {
            $($fpub $field : $type,)*
        }
        impl $name {
            $pub fn new() -> Self{
                Self{
                    $($field:crate::macroses::SameType::<$type>::new(),)*
                }
            }
        }
    }
}
