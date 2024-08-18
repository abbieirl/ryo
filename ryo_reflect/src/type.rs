pub trait Type {
    fn type_info() -> &'static TypeInfo;
}

pub struct TypeInfo {}