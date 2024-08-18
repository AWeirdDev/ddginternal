pub trait Module {
    #[allow(dead_code)]
    fn from_instance(nrj: String) -> Self;
}
