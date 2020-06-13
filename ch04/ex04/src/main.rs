pub mod compound;
pub mod scala;

use compound::check as check_compound;
use scala::check as check_scala;

fn main() {
    check_scala();
    check_compound();
}
