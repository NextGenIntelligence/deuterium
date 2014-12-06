
use std::sync::Arc;
use to_sql::{FromToSql};
use select_query::{SelectQuery, Selectable};

pub trait From { 
    fn as_sql(&self) -> &FromToSql;
    fn upcast_from(&self) -> RcFrom;
}

pub type BoxedFrom = Box<From + Send + Sync>;
pub type RcFrom = Arc<BoxedFrom>;

pub trait Table: Clone {
    fn get_table_name(&self) -> &String;
    fn get_table_alias(&self) -> &Option<String>;
}

#[deriving(Clone)]
pub struct TableDef {
    name: String,
    alias: Option<String>
}

impl TableDef {
    pub fn new(name: &str) -> TableDef {
        TableDef { name: name.to_string(), alias: None }
    }

    pub fn new_with_alias(name: &str, alias: &str) -> TableDef {
        TableDef { name: name.to_string(), alias: Some(alias.to_string()) }
    }

    pub fn alias(&self, alias: &str) -> TableDef {
        let mut table_def = self.clone();
        table_def.alias = Some(alias.to_string());
        table_def
    }
}

impl Table for TableDef {
    fn get_table_name(&self) -> &String {
        &self.name
    }

    fn get_table_alias(&self) -> &Option<String> {
        &self.alias
    }
}

impl From for TableDef {
    fn as_sql(&self) -> &FromToSql {
        self
    }

    fn upcast_from(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}

impl Selectable<()> for TableDef {}

#[deriving(Clone)]
pub struct FromSelect<T, L, M> {
    pub select: SelectQuery<T, L, M>,
    pub alias: String 
}

impl<T: Clone, L: Clone, M: Clone> From for FromSelect<T, L, M> {
    fn as_sql(&self) -> &FromToSql {
        self
    }

    fn upcast_from(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}

impl<T: Clone, L: Clone, M: Clone> Selectable<M> for FromSelect<T, L, M> {}

