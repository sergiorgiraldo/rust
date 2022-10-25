mod projects;
fn main() {
    println!("{:?}", std::env::current_dir());
    euler::timeit("p001", projects::p001::do_it);
    euler::timeit("p002", projects::p002::do_it);
    euler::timeit("p003", projects::p003::do_it);
    euler::timeit("p004", projects::p004::do_it);
    euler::timeit("p005", projects::p005::do_it);
    euler::timeit("p006", projects::p006::do_it);
    euler::timeit("p007", projects::p007::do_it);
    euler::timeit("p008", projects::p008::do_it);
    euler::timeit("p009", projects::p009::do_it);
    euler::timeit("p010", projects::p010::do_it);
    euler::timeit("p011", projects::p011::do_it);
}
