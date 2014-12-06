
use serialize::json::Json;
use time::Timespec;

use {Null};
use query::{Query, RcQuery};
use expression::{RawExpression, RawExpressionComparable};
use field::{
    NamedField,

    BoolField, BoolComparable,
    I8Field, I8Comparable,
    I16Field, I16Comparable,
    I32Field, I32Comparable,
    I64Field, I64Comparable,
    F32Field, F32Comparable,
    F64Field, F64Comparable,
    StringField, StringComparable,
    ByteListField, ByteListComparable,
    JsonField, JsonComparable,
    TimespecField, TimespecComparable
};
use to_sql::ToSql;

#[deriving(Send, Clone)]
pub struct IsQuery<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsQuery<F, T> {
    fn is(&self, val: T) -> RcQuery;
}

macro_rules! is_methods(
    ($v:ty) => (
        fn is(&self, val: T) -> RcQuery {
            IsQuery {
                field: self.clone(),
                value: val
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: $v> Query for IsQuery<$field, T> { }
        impl<T: $v> ToIsQuery<$field, T> for $field {
            is_methods!(T) 
        }
    )
)

impl_for!(BoolField, BoolComparable)
impl_for!(I8Field, I8Comparable)
impl_for!(I16Field, I16Comparable)
impl_for!(I32Field, I32Comparable)
impl_for!(I64Field, I64Comparable)
impl_for!(F32Field, F32Comparable)
impl_for!(F64Field, F64Comparable)
impl_for!(StringField, StringComparable)
impl_for!(ByteListField, ByteListComparable)
impl_for!(JsonField, JsonComparable)
impl_for!(TimespecField, TimespecComparable)
impl_for!(RawExpression, RawExpressionComparable)