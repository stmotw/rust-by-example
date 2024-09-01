// extern crate rary; // May be required for Rust 2015 edition or earlier

fn main() {
    practice_11_1_creating_a_library::public_function();

    // Error! `private_function` is private
    //practice_11_1_creating_a_library::private_function();

    practice_11_1_creating_a_library::indirect_access();
}
