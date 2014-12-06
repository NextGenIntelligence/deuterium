
use serialize::json::Json;
use time::Timespec;

use std::sync::Arc;
use std::mem;

use from::{From, RcFrom, FromSelect};
use field::{Field};
use predicate::{RcPredicate};
use to_sql::{ToSql};
use order_by::{OrderBy};
use join::{Join};

use field::{
    I8Comparable,
    I16Comparable,
    I32Comparable,
    I64Comparable,
    F32Comparable,
    F64Comparable,
    StringComparable,
    JsonComparable,
    TimespecComparable,

    I8ComparableList,
    I16ComparableList,
    I32ComparableList,
    I64ComparableList,
    F32ComparableList,
    F64ComparableList,
    StringComparableList,
    JsonComparableList,
    TimespecComparableList
};

#[deriving(Clone)]
pub enum Select {
    SelectOnly(Vec<String>),
    SelectAll
}

pub trait ToSelectQuery: Send + Sync + ToSql {
    fn upcast(self) -> RcSelectQuery {
        Arc::new(box self as BoxedSelectQuery)
    }
}

#[deriving(Clone)]
pub struct LimitOne;

#[deriving(Clone)]
pub struct LimitTwo;

#[deriving(Clone)]
pub struct LimitMany;

#[deriving(Clone)]
pub struct SelectQuery<T, L> {
    pub select: Select,
    pub from: RcFrom,
    pub where_: Option<RcPredicate>,
    pub limit: Option<uint>,
    pub offset: Option<uint>,
    pub order_by: Vec<OrderBy>,
    pub joins: Vec<Join>
}

impl<T: Clone, L: Clone> SelectQuery<T, L> {
 
    pub fn new(select: Select, from: RcFrom) -> SelectQuery<T, L> {
        SelectQuery {
            select: select,
            from: from,
            where_: None,
            limit: None,
            offset: None,
            order_by: vec![],
            joins: vec![]
        }
    }

    pub fn where_(&self, predicate: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.where_ = Some(predicate);
        query
    }

    pub fn limit(&self, limit: uint) -> SelectQuery<T, LimitOne> {
        let mut query = self.clone();
        query.limit = Some(limit);
        unsafe{ mem::transmute(query) }
    }

    pub fn first(&self) -> SelectQuery<T, LimitOne> {
        let mut query = self.clone();
        query.limit = Some(1);
        unsafe{ mem::transmute(query) }
    }

    pub fn last(&self) -> SelectQuery<T, LimitOne> {
        unimplemented!()
    }

    pub fn offset(&self, offset: uint) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.offset = Some(offset);
        query
    }

    pub fn order_by<F: Clone>(&self, field: &Field<F>) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by = vec![OrderBy::by(field)];
        query
    }

    pub fn order_by_fields<F: Clone>(&self, fields: &[&Field<F>]) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by = fields.iter().map(|f| OrderBy::by(*f)).collect();
        query
    }

    pub fn reverse_by<F: Clone>(&self, field: &Field<F>) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by = vec![OrderBy::reverse_by(field)];
        query
    }

    pub fn reverse_by_fields<F: Clone>(&self, fields: &[&Field<F>]) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by = fields.iter().map(|f| OrderBy::reverse_by(*f)).collect();
        query
    }

    pub fn order_append<F: Clone>(&self, field: &Field<F>) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by.push(OrderBy::by(field));
        query
    }

    pub fn order_prepend<F: Clone>(&self, field: &Field<F>) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by.insert(0, OrderBy::by(field));
        query
    }

    pub fn order_reverse_append<F: Clone>(&self, field: &Field<F>) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by.push(OrderBy::reverse_by(field));
        query
    }

    pub fn order_reverse_prepend<F: Clone>(&self, field: &Field<F>) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.order_by.insert(0, OrderBy::reverse_by(field));
        query
    }

    pub fn alias(&self, alias: String) -> FromSelect<T, L> {
        FromSelect { select: self.clone(), alias: alias }
    }

    pub fn inner_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::inner_join(from.upcast(), on));
        query
    }

    pub fn full_outer_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::full_outer_join(from.upcast(), on));
        query
    }

    pub fn right_outer_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::right_outer_join(from.upcast(), on));
        query
    }

    pub fn left_outer_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::left_outer_join(from.upcast(), on));
        query
    }

    pub fn full_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::full_join(from.upcast(), on));
        query
    }

    pub fn left_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::left_join(from.upcast(), on));
        query
    }

    pub fn right_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::right_join(from.upcast(), on));
        query
    }

    pub fn natural_join(&self, from: &From) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::natural_join(from.upcast()));
        query
    }
    
    pub fn natural_left_join(&self, from: &From) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::natural_left_join(from.upcast()));
        query
    }
    
    pub fn natural_right_join(&self, from: &From) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::natural_right_join(from.upcast()));
        query
    }
    
    pub fn natural_full_join(&self, from: &From) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::natural_full_join(from.upcast()));
        query
    }

    pub fn cross_join(&self, from: &From) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.joins.push(Join::cross_join(from.upcast()));
        query
    }
}

impl<T: Clone, L: Clone> ToSelectQuery for SelectQuery<T, L> { }

pub type BoxedSelectQuery = Box<ToSelectQuery + Send + Sync>;
pub type RcSelectQuery = Arc<BoxedSelectQuery>;

impl I8Comparable for SelectQuery<(i8), LimitOne> { }
impl I16Comparable for SelectQuery<(i16), LimitOne> { }
impl I32Comparable for SelectQuery<(i32), LimitOne> { }
impl I64Comparable for SelectQuery<(i64), LimitOne> { }
impl F32Comparable for SelectQuery<(f32), LimitOne> { }
impl F64Comparable for SelectQuery<(f64), LimitOne> { }
impl StringComparable for SelectQuery<(String), LimitOne> { }
impl JsonComparable for SelectQuery<(Json), LimitOne> { }
impl TimespecComparable for SelectQuery<(Timespec), LimitOne> { }

impl I8ComparableList for SelectQuery<(i8), LimitMany> { }
impl I16ComparableList for SelectQuery<(i16), LimitMany> { }
impl I32ComparableList for SelectQuery<(i32), LimitMany> { }
impl I64ComparableList for SelectQuery<(i64), LimitMany> { }
impl F32ComparableList for SelectQuery<(f32), LimitMany> { }
impl F64ComparableList for SelectQuery<(f64), LimitMany> { }
impl StringComparableList for SelectQuery<(String), LimitMany> { }
impl JsonComparableList for SelectQuery<(Json), LimitMany> { }
impl TimespecComparableList for SelectQuery<(Timespec), LimitMany> { }

