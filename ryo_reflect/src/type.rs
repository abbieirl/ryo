pub trait Type {
    fn type_info() -> &'static TypeInfo;
}

#[derive(Debug, Default)]
pub struct TypeInfo {}