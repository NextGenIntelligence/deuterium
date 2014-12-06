
use time::Timespec;

use predicate::{Predicate, RcPredicate};
use expression::{ToExpression};

use expression::{RawExpr};
use field::{
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    TimespecField,

    OptionalI8Field,
    OptionalI16Field,
    OptionalI32Field,
    OptionalI64Field,
    OptionalF32Field,
    OptionalF64Field,
    OptionalTimespecField,
};

use sql::{ToPredicateValue};

#[deriving(Clone)]
pub enum Inequality {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual
}

#[deriving(Send, Clone)]
pub struct InequalityPredicate<F, T> {
    pub field: F,
    pub value: T,
    pub inequality: Inequality
}

pub trait ToInequalityPredicate<F, T> {
    fn lt(&self, val: T) -> RcPredicate;
    fn lte(&self, val: T) -> RcPredicate;
    fn gt(&self, val: T) -> RcPredicate;
    fn gte(&self, val: T) -> RcPredicate;
}

macro_rules! inequality_methods(
    ($v:ty) => (
        fn lt(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: Inequality::LessThan
            }.upcast()
        }

        fn lte(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: Inequality::LessThanEqual
            }.upcast()
        }

        fn gt(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: Inequality::GreaterThan
            }.upcast()
        }

        fn gte(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: Inequality::GreaterThanEqual
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ty) => (
        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue + Clone> Predicate for InequalityPredicate<$field, T> { }

        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue + Clone> ToInequalityPredicate<$field, T> for $field {
            inequality_methods!(T)    
        }
    )
)

impl_for!(I8Field, i8)
impl_for!(I16Field, i16)
impl_for!(I32Field, i32)
impl_for!(I64Field, i64)
impl_for!(F32Field, f32)
impl_for!(F64Field, f64)
impl_for!(TimespecField, Timespec)

impl_for!(OptionalI8Field, Option<i8>)
impl_for!(OptionalI16Field, Option<i16>)
impl_for!(OptionalI32Field, Option<i32>)
impl_for!(OptionalI64Field, Option<i64>)
impl_for!(OptionalF32Field, Option<f32>)
impl_for!(OptionalF64Field, Option<f64>)
impl_for!(OptionalTimespecField, Option<Timespec>)

impl_for!(RawExpr, RawExpr)