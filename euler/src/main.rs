mod projects;
fn main() {
    euler::timeit("p001", || projects::p001::do_it());
    euler::timeit("p002", || projects::p002::do_it());
    euler::timeit("p003", || projects::p003::do_it());
    euler::timeit("p004", || projects::p004::do_it());
    euler::timeit("p005", || projects::p005::do_it());
    euler::timeit("p006", || projects::p006::do_it());
    euler::timeit("p007", || projects::p007::do_it());
}
