
use deuterium::*;

#[test]
fn select() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    // Type is here only for sure it is right, it can be ommited in real code
    let query: SelectQuery<(), LimitMany, ()> = jedi_table.select_all();
    assert_sql!(query, "SELECT * FROM jedi;");

    let query: SelectQuery<(String), LimitMany, ()> = jedi_table.select_1(&name);
    assert_sql!(query, "SELECT name FROM jedi;");

    let query: SelectQuery<(String), LimitMany, ()> = jedi_table.alias("j").select_1(&name);
    assert_sql!(query, "SELECT name FROM jedi AS j;");

    let query: SelectQuery<(String, bool), LimitMany, ()> = jedi_table.select_2(&name, &side);
    assert_sql!(query, "SELECT name, side FROM jedi;");
}