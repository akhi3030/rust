// error-pattern:unresolved
// xfail-test
import spam::{ham, eggs};

mod spam {
    fn ham() { }
}

fn main() { ham(); eggs(); }
