use capsolver::{Capsolver, CapsolverApi};

pub fn setup_api() -> CapsolverApi {
    let solver = CapsolverApi::new("", "https://stake1069.com/", "0x4AAAAAAAGD4gMGOTFnvupz");

    solver
}

pub fn setup_services() -> Capsolver {
    let solver = Capsolver::new("", "https://stake1069.com/", "0x4AAAAAAAGD4gMGOTFnvupz");

    solver
}
