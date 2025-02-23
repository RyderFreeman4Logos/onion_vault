pub use crate::common::traits::JsonSerializable;

// #[macro_export]
// macro_rules! print_with_line {
//         ($object:expr) => {
//             print!("########################\nLine{}:\n{}\n########################\n", line!(), $object.to_string());
//         };
//     }
// pub use crate::print_with_line;


#[macro_export]
macro_rules! print_json_with_line {
        ($object:expr) => {
            print!("########################\n{}:{}\n{}\n########################\n", file!(), line!(), $object.to_json_pretty());
        };
    }
pub use crate::print_json_with_line;


#[macro_export]
macro_rules! print_with_line {
        ($object:expr) => {
            print!("########################\n{}:{}\n{}\n########################\n", file!(), line!(), $object.to_string());
        };
    }
pub use crate::print_with_line;



#[macro_export]
macro_rules! debug_with_line {
        ($object:expr) => {
            print!("########################\n{}:{}\n{:#?}\n########################\n", file!(), line!(), $object);
        };
    }
pub use crate::debug_with_line;
