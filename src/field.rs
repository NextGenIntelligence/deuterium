use serialize::json::Json;
use time::Timespec;
use uuid::Uuid;
use std::rc::Rc;

use from::{Table};
use sql::{ToSql};
use expression::{UntypedExpression, RcExpression, BoxedExpression};

pub trait Field {
    fn name(&self) -> &str;
    fn table_name(&self) -> &str;
    fn qual(&self) -> Option<&String>;
    fn upcast_field(&self) -> RcField;
}

pub type BoxedField = Box<Field + 'static>;
pub type RcField = Rc<BoxedField>;

#[derive(Clone)]
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
        Rc::new(Box::new(self.clone()) as BoxedExpression)
    }
}

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
        Rc::new(Box::new(self.clone()) as BoxedField)
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
pub type UuidField = NamedField<Uuid>;

pub type OptionalBoolField = NamedField<Option<bool>>;
pub type OptionalI8Field = NamedField<Option<i8>>;
pub type OptionalI16Field = NamedField<Option<i16>>;
pub type OptionalI32Field = NamedField<Option<i32>>;
pub type OptionalI64Field = NamedField<Option<i64>>;
pub type OptionalF32Field = NamedField<Option<f32>>;
pub type OptionalF64Field = NamedField<Option<f64>>;
pub type OptionalStringField = NamedField<Option<String>>;
pub type OptionalByteListField = NamedField<Option<Vec<u8>>>;
pub type OptionalJsonField = NamedField<Option<Json>>;
pub type OptionalTimespecField = NamedField<Option<Timespec>>;
pub type OptionalUuidField = NamedField<Option<Uuid>>;