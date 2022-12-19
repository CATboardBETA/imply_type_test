use imply_type::imply;

fn main() {

    // Call the fn:
    foo(0i32);
}

#[imply] // <-- This is the custom attribute!!
fn foo(x: i32) -> _ {
    x + 1
}