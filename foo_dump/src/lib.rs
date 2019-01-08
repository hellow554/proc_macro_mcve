use foo::{P1, P2};

pub trait Dumper {
    fn dump_p1(&mut self, value: &P1);
    fn dump_p2(&mut self, value: &P2);
}

pub trait Dump {
    fn dump<D: Dumper>(&self, d: D);
}

impl Dump for P1 {
    fn dump<D: Dumper>(&self, d: D) {
        d.dump_p1(self);
    }
}

impl Dump for P2 {
    fn dump<D: Dumper>(&self, d: D) {
        d.dump_p2(self);
    }
}
