use super::super::sql;
use super::super::expression;
use super::super::field;

use super::{ToAbstractPredicate};

#[derive(Clone)]
pub struct IsNullPredicate<F> {
    pub field: F,
    pub null: bool
}

pub trait ToIsNullPredicate {
    fn is_null(&self) -> super::RcPredicate;
    fn not_null(&self) -> super::RcPredicate;
}

impl<F> super::Predicate for IsNullPredicate<F> where F: sql::ToPredicateValue {}

impl<T> ToIsNullPredicate for field::NamedField<Option<T>> where T: sql::ToPredicateValue + Clone {
    fn is_null(&self) -> super::RcPredicate {
        IsNullPredicate { field: self.clone(), null: true }.upcast()
    }

    fn not_null(&self) -> super::RcPredicate {
        IsNullPredicate { field: self.clone(), null: false }.upcast()
    }
}

impl ToIsNullPredicate for expression::RawExpr {
    fn is_null(&self) -> super::RcPredicate {
        IsNullPredicate { field: self.clone(), null: true }.upcast()
    }

    fn not_null(&self) -> super::RcPredicate {
        IsNullPredicate { field: self.clone(), null: false }.upcast()
    }
}