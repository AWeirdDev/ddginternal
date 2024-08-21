pub trait Module {
    fn from_instance(nrj: String) -> Option<Self>
    where
        Self: Sized;
}
