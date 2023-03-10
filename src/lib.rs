use std::{
    collections::HashMap,
    io::{self},
};

pub mod iterate_struct;
use serde::Serialize;

use crate::iterate_struct::iterate_struct;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
fn sql_injection_prevention(query: String) -> String {
    query
        .replace("'", "''")
        .replace(";", "")
        .replace("--", "")
        .replace("/*", "")
        .replace("*/", "")
}

#[allow(non_snake_case)]
pub trait QueryBuilder {
    fn new() -> Self;
    fn table(table: &str) -> Self;
    fn AND_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn AND(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn FROM(&mut self, table: &str) -> &mut Self;
    fn INSERT_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self;
    fn INSERT<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Serialize;
    fn LIMIT(&mut self, limit: u32) -> &mut Self;
    fn OFFSET(&mut self, limit: u32) -> &mut Self;
    fn OR_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn OR(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn ORDER_BY(&mut self, columns: &[&str]) -> &mut Self;
    fn RETURNING(&mut self, columns: &[&str]) -> &mut Self;
    fn SELECT_DISTINCT(&mut self, columns: &[&str]) -> &mut Self;
    fn SELECT(&mut self, columns: &[&str]) -> &mut Self;
    fn SET(&mut self, columns: &[&str]) -> &mut Self;
    fn UPDATE_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self;
    fn UPDATE<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Serialize;
    fn WHERE_AND<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Serialize;
    fn WHERE_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn WHERE(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn to_string(&self) -> io::Result<String>;
    fn build(&self) -> io::Result<String>;
}

pub struct SqlQueryBuilder {
    query: String,
    table: String,
    error_message: Option<String>,
}

impl QueryBuilder for SqlQueryBuilder {
    fn new() -> Self {
        Self {
            query: String::new(),
            table: String::new(),
            error_message: None,
        }
    }

    fn table(table: &str) -> Self {
        let table = sql_injection_prevention(table.to_string());
        Self {
            query: String::new(),
            table,
            error_message: None,
        }
    }

    fn AND_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(operand.to_string());
        let operator = sql_injection_prevention(operator.to_string());
        let result = sql_injection_prevention(result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("AND NOT ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn AND(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(operand.to_string());
        let operator = sql_injection_prevention(operator.to_string());
        let result = sql_injection_prevention(result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("AND ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn FROM(&mut self, table: &str) -> &mut Self {
        let table = sql_injection_prevention(table.to_string());
        self.query.push_str("FROM ");
        self.query.push_str(&table);
        self.query.push_str(" ");
        self
    }

    fn INSERT_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self {
        let mut columns = String::new();
        let mut values = String::new();
        for (column, value) in columns_and_values {
            columns.push_str(&sql_injection_prevention(column.to_string()));
            columns.push_str(", ");
            values.push_str("'");
            values.push_str(&sql_injection_prevention(value.to_string()));
            values.push_str("', ");
        }
        // let table = sql_injection_prevention(table.to_string());
        self.query.push_str("INSERT INTO ");
        self.query.push_str(&self.table);
        self.query.push_str(" ");
        self.query.push_str("(");
        self.query.push_str(&columns[..columns.len() - 2]);
        self.query.push_str(") ");
        self.query.push_str("VALUES ");
        self.query.push_str("(");
        self.query.push_str(&values[..values.len() - 2]);
        self.query.push_str(") ");
        self
    }

    fn INSERT<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Serialize,
    {
        let iterable: HashMap<String, String> = iterate_struct(&columns);

        let mut columns = String::new();
        let mut values = String::new();

        for (column, value) in &iterable {
            columns.push_str(&sql_injection_prevention(column.to_string()));
            columns.push_str(", ");
            values.push_str("'");
            values.push_str(&sql_injection_prevention(value.to_string()));
            values.push_str("', ");
        }

        self.query.push_str("INSERT INTO ");
        self.query.push_str(&self.table);
        self.query.push_str(" ");
        self.query.push_str("(");
        self.query.push_str(&columns[..columns.len() - 2]);
        self.query.push_str(") ");
        self.query.push_str("VALUES ");
        self.query.push_str("(");
        self.query.push_str(&values[..values.len() - 2]);
        self.query.push_str(") ");
        self
    }

    fn LIMIT(&mut self, limit: u32) -> &mut Self {
        self.query.push_str("LIMIT ");
        self.query.push_str(&limit.to_string());
        self.query.push_str(" ");
        self
    }

    fn OFFSET(&mut self, limit: u32) -> &mut Self {
        self.query.push_str("OFFSET ");
        self.query.push_str(&limit.to_string());
        self.query.push_str(" ");
        self
    }

    fn OR_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(operand.to_string());
        let operator = sql_injection_prevention(operator.to_string());
        let result = sql_injection_prevention(result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("OR NOT ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn OR(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(operand.to_string());
        let operator = sql_injection_prevention(operator.to_string());
        let result = sql_injection_prevention(result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("OR ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn ORDER_BY(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("ORDER BY ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn RETURNING(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("RETURNING ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn SELECT_DISTINCT(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("SELECT DISTINCT ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn SELECT(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("SELECT ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn SET(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("SET ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn UPDATE_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self {
        let mut sets = String::new();
        for (column, value) in columns_and_values {
            sets.push_str(&sql_injection_prevention(column.to_string()));
            sets.push_str(" = ");
            sets.push_str("'");
            sets.push_str(&sql_injection_prevention(value.to_string()));
            sets.push_str("', ");
        }
        if sets.len() <= 2 {
            panic!("No columns and values provided");
        }
        // let table = sql_injection_prevention(table.to_string());
        self.query.push_str("UPDATE ");
        self.query.push_str(&self.table);
        self.query.push_str(" SET ");
        self.query.push_str(&sets[..sets.len() - 2]);
        self.query.push_str(" ");
        self
    }

    fn UPDATE<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Serialize,
    {
        let iterable: HashMap<String, String> = iterate_struct(&columns);

        let mut values: Vec<(&str, &str)> = Vec::new();
        for item in &iterable {
            if !(item.1 == "" || item.1 == "null") {
                values.push((&item.0, &item.1));
            }
        }

        let mut sets = String::new();
        for (column, value) in values {
            sets.push_str(&sql_injection_prevention(column.to_string()));
            sets.push_str(" = ");
            sets.push_str("'");
            sets.push_str(&sql_injection_prevention(value.to_string()));
            sets.push_str("', ");
        }
        if sets.len() <= 2 {
            panic!("No columns and values provided");
        }
        // let table = sql_injection_prevention(table.to_string());
        self.query.push_str("UPDATE ");
        self.query.push_str(&self.table);
        self.query.push_str(" SET ");
        self.query.push_str(&sets[..sets.len() - 2]);
        self.query.push_str(" ");
        self
    }

    fn WHERE_AND<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Serialize,
    {
        let iterable: HashMap<String, String> = iterate_struct(&columns);

        if iterable.len() == 0 {
            return self;
        }

        let mut columns = String::new();

        for (column, value) in &iterable {
            columns.push_str(&sql_injection_prevention(column.to_string()));
            columns.push_str(" = '");
            columns.push_str(&sql_injection_prevention(value.to_string()));
            columns.push_str("' AND ");
        }

        if columns.len() <= 5 {
            panic!("No columns and values provided");
        }

        self.query.push_str("WHERE ");
        self.query.push_str(&columns[..columns.len() - 5]);
        self.query.push_str(" ");
        self
    }

    fn WHERE_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(operand.to_string());
        let operator = sql_injection_prevention(operator.to_string());
        let result = sql_injection_prevention(result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("WHERE NOT ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn WHERE(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(operand.to_string());
        let operator = sql_injection_prevention(operator.to_string());
        let result = sql_injection_prevention(result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("WHERE ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn to_string(&self) -> io::Result<String> {
        if self.query.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No query has been built",
            ))
        } else {
            Ok(self.query.clone().trim().to_owned() + ";")
        }
    }

    fn build(&self) -> io::Result<String> {
        if self.query.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No query has been built",
            ))
        } else {
            Ok(self.query.clone().trim().to_owned() + ";")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_insert_and_return() {
        let query = SqlQueryBuilder::table("users")
            .INSERT_AS_SLICE(&[("id", "1"), ("name", "John")])
            .RETURNING(&["*"])
            .build()
            .unwrap();

        assert_eq!(
            query,
            "INSERT INTO users (id, name) VALUES ('1', 'John') RETURNING *;"
        );
    }

    #[test]
    fn should_insert_and_return_from_struct() {
        #[derive(Serialize)]
        struct User {
            id: Option<i32>,
            name: Option<String>,
        }

        let user = User {
            id: Some(1),
            name: Some("John".to_string()),
        };

        let query = SqlQueryBuilder::table("users")
            .INSERT::<User>(&user)
            .RETURNING(&["*"])
            .build()
            .unwrap();

        assert_eq!(
            query,
            "INSERT INTO users (id, name) VALUES ('1', 'John') RETURNING *;"
        );
    }

    #[test]
    fn should_select_where_id_equal_1() {
        let query = SqlQueryBuilder::new()
            .SELECT(&["id", "name"])
            .FROM("users")
            .WHERE("id", "=", "1")
            .build()
            .unwrap();

        assert_eq!(query, "SELECT id, name FROM users WHERE id = '1';");
    }

    #[test]
    fn should_select_where_id_equal_1_and_name_equal_john() {
        let query = SqlQueryBuilder::new()
            .SELECT(&["id", "name"])
            .FROM("users")
            .WHERE("id", "=", "1")
            .AND("name", "=", "John")
            .build()
            .unwrap();

        assert_eq!(
            query,
            "SELECT id, name FROM users WHERE id = '1' AND name = 'John';"
        );
    }

    #[test]
    fn should_update_users_set_name_equal_john_where_id_equal_1() {
        #[derive(Serialize)]
        struct User {
            name: Option<String>,
            id: Option<String>,
        }

        let user = User {
            name: Some("John".to_string()),
            id: None,
        };

        let query = SqlQueryBuilder::table("users")
            .UPDATE::<User>(&user)
            .WHERE("id", "=", "1")
            .build()
            .unwrap();

        assert_eq!(query, "UPDATE users SET name = 'John' WHERE id = '1';");
    }

    #[test]
    fn should_use_multiple_where_and() {
        #[derive(Serialize)]
        struct User {
            name: Option<String>,
            id: Option<String>,
        }

        let user = User {
            name: Some("John".to_string()),
            id: None,
        };

        let query = SqlQueryBuilder::new()
            .SELECT(&["*"])
            .WHERE_AND(&user)
            .build()
            .unwrap();

        assert_eq!(query, "SELECT * WHERE name = 'John';");
    }
}

// ---- tests::should_update_users_set_name_equal_john_where_id_equal_1 stdout ----
// thread 'tests::should_update_users_set_name_equal_john_where_id_equal_1' panicked at 'called `Result::unwrap()` on an `Err` value: Error("invalid type: string \"Sequence [ Sequence [ \\\"name\\\", \\\"John\\\", ], ]\", expected a map")', src/iterate_struct.rs:11:41
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ---- tests::should_update_users_set_name_equal_john_where_id_equal_1 stdout ----
// thread 'tests::should_update_users_set_name_equal_john_where_id_equal_1' panicked at 'assertion failed: `(left == right)`
//   left: `"UPDATE users SET name = 'John', id = '1' WHERE id = '1';"`,
//  right: `"UPDATE users SET name = 'John' WHERE id = '1';"`',
//           UPDATE users SET name = 'John' WHERE id = '1';");
