use foo_core::{P1, P2};
use foo_derive::Dump;

#[derive(Dump)]
struct Bar {
    primitive_one: P1,
    primitive_two: P2,
}
