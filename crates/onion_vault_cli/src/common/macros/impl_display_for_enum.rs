// #[macro_export]
// macro_rules! impl_display_for_enum {
//     ($enum_name:ident {
//         $($variant:ident => $str:expr,)*
//     }) => {
//         impl fmt::Display for $enum_name {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 match self {
//                     $($enum_name::$variant => write!(f, "{}", $str),)*
//                 }
//             }
//         }
//     };
// }
