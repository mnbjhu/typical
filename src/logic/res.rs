use crate::ty::bound::Bound;

pub enum CheckRes {
    Yes,
    No,
    OnlyIf(Vec<Bound>),
}
