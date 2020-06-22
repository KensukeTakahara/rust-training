pub mod new_type;
pub mod std_lib;

use new_type::check as new_type_check;
use std_lib::check as std_lib_check;

fn main() {
    std_lib_check();
    new_type_check();
}
