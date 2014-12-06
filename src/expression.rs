
use to_sql::{ToSql};
use std::sync::Arc;

use serialize::json::Json;
use time::Timespec;
use std::mem;

use field::{
    StringField,
    BoolField,
    ByteListField,
    JsonField,
    TimespecField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
};

pub trait Expression<T> for Sized?: UntypedExpression {}
pub trait ListExpression<T> for Sized?: UntypedExpression {}

pub trait UntypedExpression for Sized? {
    fn expression_as_sql(&self) -> &ToSql;
    fn upcast_expression(&self) -> RcExpression;
}

pub type BoxedExpression = Box<UntypedExpression + Send + Sync>;
pub type RcExpression = Arc<BoxedExpression>;

#[deriving(Clone)]
pub enum ExprValue<T> {
    ExpressionValue {
        expression: RcExpression
    },
    DefaultValue
}

pub trait ToExprValue<T> for Sized? {
    fn to_expr_val(&self) -> ExprValue<T>;
}

impl<T> ExprValue<T> {
    pub fn new(exp: &Expression<T>) -> ExprValue<T> {
        ExpressionValue {
            expression: exp.upcast_expression()
        }
    }
}

#[deriving(Clone)]
pub struct RawExpr {
    pub content: String
}

impl RawExpr {
    pub fn new(content: String) -> RawExpr { 
        RawExpr {
            content: content
        }
    }
}

macro_rules! impl_expression_for(
    ($t:ty) => (
        impl UntypedExpression for $t {
            fn expression_as_sql(&self) -> &ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                Arc::new(box self.clone() as BoxedExpression)
            }
        }

        impl Expression<$t> for $t {
            
        }

        impl UntypedExpression for Vec<$t> {
            fn expression_as_sql(&self) -> &ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                Arc::new(box self.clone() as BoxedExpression)
            }
        }

        impl ListExpression<$t> for Vec<$t> {
            
        }
    )
)

impl<'a, 'b, T> ToExprValue<T> for &'a Expression<T> + 'b {
    fn to_expr_val(&self) -> ExprValue<T> {
        ExprValue::new(*self)
    }   
}

impl_expression_for!(bool)
impl_expression_for!(Option<bool>)
impl_expression_for!(i8)
impl_expression_for!(Option<i8>)
impl_expression_for!(i16)
impl_expression_for!(Option<i16>)
impl_expression_for!(i32)
impl_expression_for!(Option<i32>)
impl_expression_for!(i64)
impl_expression_for!(Option<i64>)
impl_expression_for!(f32)
impl_expression_for!(Option<f32>)
impl_expression_for!(f64)
impl_expression_for!(Option<f64>)
impl_expression_for!(String)
impl_expression_for!(Option<String>)
impl_expression_for!(Vec<u8>)
impl_expression_for!(Option<Vec<u8>>)
impl_expression_for!(Json)
impl_expression_for!(Option<Json>)
impl_expression_for!(Timespec)
impl_expression_for!(Option<Timespec>)
impl_expression_for!(RawExpr)
impl_expression_for!(Option<RawExpr>)

pub trait ToExpression<T> for Sized?: UntypedExpression {
    fn as_expr(&self) -> &Expression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

pub trait ToListExpression<T> for Sized?: UntypedExpression {
    fn as_expr(&self) -> &ListExpression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

impl ToListExpression<bool> for Vec<bool> {}
impl ToListExpression<i8> for Vec<i8> {}
impl ToListExpression<i16> for Vec<i16> {}
impl ToListExpression<i32> for Vec<i32> {}
impl ToListExpression<i64> for Vec<i64> {}
impl ToListExpression<f32> for Vec<f32> {}
impl ToListExpression<f64> for Vec<f64> {}
impl ToListExpression<String> for Vec<String> {}
impl ToListExpression<Vec<u8>> for Vec<Vec<u8>> {}
impl ToListExpression<Json> for Vec<Json> {}
impl ToListExpression<Timespec> for Vec<Timespec> {}

macro_rules! cast_numbers(
    ($comp:ty) => (
        impl $comp for i8 {}
        impl $comp for i16 {}
        impl $comp for i32 {}
        impl $comp for i64 {}
        impl $comp for f32 {}
        impl $comp for f64 {}
        impl $comp for Option<i8> {}
        impl $comp for Option<i16> {}
        impl $comp for Option<i32> {}
        impl $comp for Option<i64> {}
        impl $comp for Option<f32> {}
        impl $comp for Option<f64> {}
        impl $comp for I8Field {} 
        impl $comp for I16Field {} 
        impl $comp for I32Field {} 
        impl $comp for I64Field {} 
        impl $comp for F32Field {} 
        impl $comp for F64Field {} 
        impl $comp for RawExpr {}
    )
)

impl ToExpression<String> for String {}
impl ToExpression<String> for Option<String> {}
impl ToExpression<String> for StringField {}
impl ToExpression<String> for RawExpr {}

cast_numbers!(ToExpression<i8>)
cast_numbers!(ToExpression<i16>)
cast_numbers!(ToExpression<i32>)
cast_numbers!(ToExpression<i64>)
cast_numbers!(ToExpression<f32>)
cast_numbers!(ToExpression<f64>)

impl ToExpression<bool> for bool {}
impl ToExpression<bool> for Option<bool> {}
impl ToExpression<bool> for BoolField {} 
impl ToExpression<bool> for RawExpr {} 

impl ToExpression<Vec<u8>> for Vec<u8> {}
impl ToExpression<Vec<u8>> for Option<Vec<u8>> {}
impl ToExpression<Vec<u8>> for ByteListField {}
impl ToExpression<Vec<u8>> for RawExpr {}

impl ToExpression<Json> for Json {}
impl ToExpression<Json> for Option<Json> {}
impl ToExpression<Json> for JsonField {}
impl ToExpression<Json> for RawExpr {}

impl ToExpression<Timespec> for Timespec {}
impl ToExpression<Timespec> for Option<Timespec> {}
impl ToExpression<Timespec> for TimespecField {}
impl ToExpression<Timespec> for RawExpr {}

impl ToExpression<RawExpr> for bool {}
impl ToExpression<RawExpr> for i8 {}
impl ToExpression<RawExpr> for i16 {}
impl ToExpression<RawExpr> for i32 {}
impl ToExpression<RawExpr> for i64 {}
impl ToExpression<RawExpr> for f32 {}
impl ToExpression<RawExpr> for f64 {}
impl ToExpression<RawExpr> for Vec<u8> {}
impl ToExpression<RawExpr> for String {}
impl ToExpression<RawExpr> for Json {}
impl ToExpression<RawExpr> for Timespec {}
impl ToExpression<RawExpr> for Option<bool> {}
impl ToExpression<RawExpr> for Option<i8> {}
impl ToExpression<RawExpr> for Option<i16> {}
impl ToExpression<RawExpr> for Option<i32> {}
impl ToExpression<RawExpr> for Option<i64> {}
impl ToExpression<RawExpr> for Option<f32> {}
impl ToExpression<RawExpr> for Option<f64> {}
impl ToExpression<RawExpr> for Option<Vec<u8>> {}
impl ToExpression<RawExpr> for Option<String> {}
impl ToExpression<RawExpr> for Option<Json> {}
impl ToExpression<RawExpr> for Option<Timespec> {}
impl ToExpression<RawExpr> for BoolField {} 
impl ToExpression<RawExpr> for I8Field {} 
impl ToExpression<RawExpr> for I16Field {} 
impl ToExpression<RawExpr> for I32Field {} 
impl ToExpression<RawExpr> for I64Field {} 
impl ToExpression<RawExpr> for F32Field {} 
impl ToExpression<RawExpr> for F64Field {} 
impl ToExpression<RawExpr> for StringField {} 
impl ToExpression<RawExpr> for JsonField {} 
impl ToExpression<RawExpr> for ByteListField {} 
impl ToExpression<RawExpr> for TimespecField {} 