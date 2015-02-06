use std::mem;

use from::{Table, RcTable};
use field::{Field, RcField};
use select_query::{SelectQuery, LimitMany, Select, NoResult};
use expression::{
    Expression, ToExpression, UntypedExpression,     
    ExprValue, ToExprValue};

#[allow(dead_code)]
#[derive(Clone)]
pub enum Insert<T, V, M> {
    DefaultValues,
    Values(Vec<V>),
    UntypedValues(Vec<Vec<ExprValue<()>>>),
    FromSelect(SelectQuery<T, LimitMany, M>)
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct InsertQuery<T, V, M, RT, RL> {
    into: RcTable,
    cols: Option<Vec<RcField>>,
    values: Insert<T, V, M>,
    returning: Option<Select>
}

macro_rules! insert {
    ($name:ident, $(($t:ident, $arg:ident)),+) => (
        // FIXME: Make this public after https://github.com/rust-lang/rust/issues/17635
        fn $name<$($t:Clone,)+>(&self, $($arg: &NamedField<$t>,)+) -> InsertQuery<($($t,)+), ($(ExprValue<$t>,)+), M, (), ()> {
            let mut cols = vec![];
            $(cols.push((*$arg).upcast_field());)+
            let mut query = InsertQuery::new(self);
            query.cols = Some(cols);
            query
        }
    )
}

macro_rules! insertable {
    () => (
        pub trait Insertable<M: Clone>: Table {   
            // FIXME: It doesn't work for now because of :
            //        [Cannot use Macros in Trait Bodies](https://github.com/rust-lang/rust/issues/11403)
            //        [Impossible to have a macro expand to `pub` method](https://github.com/rust-lang/rust/issues/17436)
            // FIXME: Rewrite after https://github.com/rust-lang/rfcs/issues/376:
            //        Draft RFC: variadic generics
            // insert!(insert_1, (T0, _t0))
            // insert!(insert_2, (T0, _t0), (T1, _t1))
            // insert!(insert_3, (T0, _t0), (T1, _t1), (T2, _t2))
            // insert!(insert_4, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3))
            // insert!(insert_5, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4))
            // insert!(insert_6, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5))
            // insert!(insert_7, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6))
            // insert!(insert_8, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7))
            // insert!(insert_9, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8))
            // insert!(insert_10, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9))
            // insert!(insert_11, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10))
            // insert!(insert_12, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11))
        
            fn insert_all(&self) -> InsertQuery<(), (), M, (), ()> {
                 InsertQuery::new(self)
            }

            fn insert_fields(&self, fields: &[&Field]) -> InsertQuery<(), (), M, (), ()> {
                let mut cols = vec![];
                for field in fields.iter() {
                    cols.push(field.upcast_field())
                }
                InsertQuery::new_with_cols(self, cols)
            }
        }
    )
}


insertable!();

#[allow(dead_code)]
impl<T: Clone, V: Clone, M: Clone, RT: Clone, RL: Clone> InsertQuery<T, V, M, RT, RL> {
    
    pub fn new(into: &Table) -> InsertQuery<T, V, M, RT, RL> {
        InsertQuery {
            into: into.upcast_table(),
            cols: None,
            values: Insert::DefaultValues,
            returning: None
        }
    }

    pub fn new_with_cols(into: &Table, cols: Vec<RcField>) -> InsertQuery<T, V, M, RT, RL> {
        InsertQuery {
            into: into.upcast_table(),
            cols: Some(cols),
            values: Insert::DefaultValues,
            returning: None
        }
    }

    pub fn get_into(&self) -> &RcTable { &self.into }
    pub fn get_cols(&self) -> &Option<Vec<RcField>> { &self.cols }
    pub fn get_values(&self) -> &Insert<T, V, M> { &self.values }
    pub fn get_returning(&self) -> &Option<Select> { &self.returning }

    pub fn push(&mut self, value: V) {

        let mut reassign = false;
        match &self.values {
            &Insert::DefaultValues | &Insert::FromSelect(_) => {
                reassign = true;
            },
            _ => ()
        }

        if reassign {
            self.values = Insert::Values(vec![value])
        } else {
            match &mut self.values {
                &Insert::Values(ref mut values) => {
                    values.push(value)
                },
                _ => ()
            }
        }
    }

    pub fn push_untyped(&mut self, values: &[&ToExpression<()>]) {
        let mut reassign = false;
        match &self.values {
            &Insert::DefaultValues | &Insert::FromSelect(_) => {
                reassign = true;
            },
            _ => ()
        }

        let values_vec = values.iter().map(|v| v.as_expr().to_expr_val()).collect();

        if reassign {
            self.values = Insert::UntypedValues(vec![values_vec])
        } else {
            match &mut self.values {
                &Insert::UntypedValues(ref mut values) => {
                    values.push(values_vec)
                },
                _ => ()
            }
        }
    }

    pub fn from_select(&self, select: SelectQuery<T, LimitMany, M>) -> InsertQuery<T, V, M, RT, RL> {
        with_clone!(self, query, query.values = Insert::FromSelect(select))
    }

}

impl<T: Clone, V: Clone, M: Clone, RT, RL> InsertQuery<T, V, M, RT, RL> {
    pub fn returning_1<T1: Clone>(mut self, field: &Expression<T1>) -> InsertQuery<T, V, M, (T1), LimitMany> {
        self.returning = Some(Select::Only(vec![field.upcast_expression()]));
        unsafe{ mem::transmute(self) }
    }

    pub fn returning_2<T1: Clone, T2: Clone>(mut self, field1: &Expression<T1>, field2: &Expression<T2>) -> InsertQuery<T, V, M, (T1, T2), LimitMany> {
        self.returning = Some(Select::Only(vec![field1.upcast_expression(), field2.upcast_expression()]));
        unsafe{ mem::transmute(self) }
    }

    pub fn returning(mut self, fields: &[&UntypedExpression]) -> InsertQuery<T, V, M, (), LimitMany> {
        self.returning = Some(Select::Only(fields.iter().map(|f| f.upcast_expression()).collect()));
        unsafe{ mem::transmute(self) }
    }

    pub fn returning_all(mut self) -> InsertQuery<T, V, M, (), LimitMany> {
        self.returning = Some(Select::All);
        unsafe{ mem::transmute(self) }
    }
    
    pub fn no_returning(mut self) -> InsertQuery<T, V, M, (), NoResult> {
        self.returning = None;
        unsafe{ mem::transmute(self) }
    }
}
