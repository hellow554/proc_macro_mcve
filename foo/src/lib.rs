use foo_derive::Dump;

struct P1;
struct P2;

#[derive(Dump)]
struct Bar {
    primitive_one: P1,
    primitive_two: P2,
}
