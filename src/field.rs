use serialize::json::Json;
use time::Timespec;
use std::sync::Arc;

use to_sql::{ToPredicateValue};
use raw_expression::{RawExpression};
use from::{Table};
use to_sql::{ToSql};
use expression::{Expression, UntypedExpression, RcExpression, BoxedExpression};

pub trait Field {
    fn name(&self) -> &str;
    fn table_name(&self) -> &str;
    fn qual(&self) -> Option<&String>;
    fn upcast_field(&self) -> RcField;
}

pub type BoxedField = Box<Field + Send + Sync>;
pub type RcField = Arc<BoxedField>;

#[deriving(Clone)]
pub struct NamedField<T> {
    pub name: String,
    pub table_name: String,
    pub qual: Option<String>
}

impl<T: Clone> NamedField<T> {
    pub fn new(name: &str, table_name: &str) -> NamedField<T>  {
        NamedField { 
            name: name.to_string(), 
            table_name: table_name.to_string(), 
            qual: None 
        }
    }

    pub fn new_qual(name: &str, table_name: &str, qual: &str) -> NamedField<T>  {
        NamedField { 
            name: name.to_string(), 
            table_name: table_name.to_string(),
            qual: Some(qual.to_string()) 
        }
    }

    pub fn field_of(name: &str, table: &Table) -> NamedField<T> {
        NamedField { 
            name: name.to_string(), 
            table_name: table.get_table_name().to_string(),
            qual: table.get_table_alias().as_ref().map(|v| v.to_string())
        }
    }

    pub fn qual(&self) -> NamedField<T> {
        let mut field = self.clone();
        field.qual = Some(self.table_name.to_string());
        field
    }

    pub fn qual_with(&self, qual: &str) -> NamedField<T> {
        let mut field = self.clone();
        field.qual = Some(qual.to_string());
        field
    }

    pub fn qual_for(&self, table: &Table) -> NamedField<T> {
        let mut field = self.clone();
        field.qual = table.get_table_alias().as_ref().map(|v| v.to_string());
        field
    }
}

impl<T: Clone> UntypedExpression for NamedField<T> {
    fn expression_as_sql(&self) -> &ToSql {
        self
    }

    fn upcast_expression(&self) -> RcExpression {
        Arc::new(box self.clone() as BoxedExpression)
    }
}

impl<T: Clone> Expression<T> for NamedField<T> {}

impl<T: Clone> Field for NamedField<T> {
    fn name(&self) -> &str {
        self.name.as_slice()
    }

    fn table_name(&self) -> &str {
        self.table_name.as_slice()
    }

    fn qual(&self) -> Option<&String> {
        self.qual.as_ref()
    }

    fn upcast_field(&self) -> RcField {
        Arc::new(box self.clone() as BoxedField)
    }
}

pub type BoolField = NamedField<bool>;
pub type I8Field = NamedField<i8>;
pub type I16Field = NamedField<i16>;
pub type I32Field = NamedField<i32>;
pub type I64Field = NamedField<i64>;
pub type F32Field = NamedField<f32>;
pub type F64Field = NamedField<f64>;
pub type StringField = NamedField<String>;
pub type ByteListField = NamedField<Vec<u8>>;
pub type JsonField = NamedField<Json>;
pub type TimespecField = NamedField<Timespec>;

pub trait BoolComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I8Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I16Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I32Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I64Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait F32Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait F64Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait StringComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait ByteListComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait JsonComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait TimespecComparable: Send + Clone + Sync + ToPredicateValue { }

pub trait BoolComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait I8ComparableList: Send + Clone + Sync + ToPredicateValue{ }
pub trait I16ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait I32ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait I64ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait F32ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait F64ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait StringComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait ByteListComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait JsonComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait TimespecComparableList: Send + Clone + Sync + ToPredicateValue { }

impl BoolComparableList for Vec<bool> {}
impl I8ComparableList for Vec<i8> {}
impl I16ComparableList for Vec<i16> {}
impl I32ComparableList for Vec<i32> {}
impl I64ComparableList for Vec<i64> {}
impl F32ComparableList for Vec<f32> {}
impl F64ComparableList for Vec<f64> {}
impl StringComparableList for Vec<String> {}
impl StringComparableList for Vec<&'static str> {}
impl ByteListComparableList for Vec<Vec<u8>> {}
impl JsonComparableList for Vec<Json> {}
impl TimespecComparableList for Vec<Timespec> {}



macro_rules! number_comparable(
    ($comp:ty) => (
        impl $comp for i8 {}
        impl $comp for i16 {}
        impl $comp for i32 {}
        impl $comp for int {}
        impl $comp for uint {}
        impl $comp for i64 {}
        impl $comp for f32 {}
        impl $comp for f64 {}
        impl $comp for I8Field {} 
        impl $comp for I16Field {} 
        impl $comp for I32Field {} 
        impl $comp for I64Field {} 
        impl $comp for F32Field {} 
        impl $comp for F64Field {} 
        impl $comp for RawExpression {}
    )
)

number_comparable!(I8Comparable)
number_comparable!(I16Comparable)
number_comparable!(I32Comparable)
number_comparable!(I64Comparable)
number_comparable!(F32Comparable)
number_comparable!(F64Comparable)

impl StringComparable for String {}
impl StringComparable for &'static str {}
impl StringComparable for StringField {}
impl StringComparable for RawExpression {}

impl BoolComparable for bool {}
impl BoolComparable for BoolField {} 
impl BoolComparable for RawExpression {} 

impl ByteListComparable for Vec<u8> {}
impl ByteListComparable for ByteListField {}
impl ByteListComparable for RawExpression {}

impl JsonComparable for Json {}
impl JsonComparable for JsonField {}
impl JsonComparable for RawExpression {}

impl TimespecComparable for Timespec {}
impl TimespecComparable for TimespecField {}
impl TimespecComparable for RawExpression {}