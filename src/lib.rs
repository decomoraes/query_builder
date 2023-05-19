use std::{
    collections::HashMap,
    io::{self},
};

pub mod iterate_struct;
use struct_iterable::Iterable;

use crate::iterate_struct::iterate_struct;

fn sql_injection_prevention(query: &str) -> String {
    let mut sanitized_query = String::new();
    let mut in_comment = false;

    for c in query.chars() {
        if in_comment {
            if c == '*' && query.trim_start().starts_with("*/") {
                in_comment = false;
            }
            continue;
        }
        match c {
            '\\' => sanitized_query.push_str("\\\\"),
            '\'' => sanitized_query.push_str("\\'"),
            '\"' => sanitized_query.push_str("\\\""),
            '\n' => sanitized_query.push_str("\\n"),
            '\r' => sanitized_query.push_str("\\r"),
            '\t' => sanitized_query.push_str("\\t"),
            '-' => {
                if query.trim_start().starts_with("--") {
                    break;
                } else {
                    sanitized_query.push(c);
                }
            }
            '/' => {
                if query.trim_start().starts_with("/*") {
                    in_comment = true;
                } else {
                    sanitized_query.push(c);
                }
            }
            _ => sanitized_query.push(c),
        }
    }
    sanitized_query
}

#[allow(non_snake_case)]
pub trait QueryBuilder: Clone + Default + Sized {
    /// Constructs a new `SqlQueryBuilder`.
    fn new() -> Self;
    /// Constructs a new `SqlQueryBuilder` with a table.
    fn table(table: &str) -> Self;
    /// Adds an AND NOT clause to the SQL query.
    fn AND_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    /// Adds an AND clause to the SQL query.
    fn AND(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    /// Adds a FROM clause to the SQL query.
    fn FROM(&mut self, table: &str) -> &mut Self;
    /// Inserts a slice of columns and values into the SQL query.
    fn INSERT_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self;
    /// Inserts an iterable of columns into the SQL query.
    fn INSERT<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Iterable;
    /// Adds a JOIN clause to the SQL query.
    fn JOIN(&mut self, table: &str, column1: &str, operator: &str, column2: &str) -> &mut Self;
    /// Adds a LIMIT clause to the SQL query.
    fn LIMIT(&mut self, limit: u32) -> &mut Self;
    /// Adds an OFFSET clause to the SQL query.
    fn OFFSET(&mut self, limit: u32) -> &mut Self;
    /// Adds an OR NOT clause to the SQL query.
    fn OR_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    /// Adds an OR clause to the SQL query.
    fn OR(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    /// Adds an ORDER BY clause to the SQL query.
    fn ORDER_BY(&mut self, columns: &[&str]) -> &mut Self;
    /// Adds a RETURNING clause to the SQL query.
    fn RETURNING(&mut self, columns: &[&str]) -> &mut Self;
    /// Adds a SELECT DISTINCT clause to the SQL query.
    fn SELECT_DISTINCT(&mut self, columns: &[&str]) -> &mut Self;
    /// Adds a SELECT clause to the SQL query.
    fn SELECT(&mut self, columns: &[&str]) -> &mut Self;
    /// Adds a SET clause to the SQL query.
    fn SET(&mut self, columns: &[&str]) -> &mut Self;
    /// Adds a UPDATE_AS_SLICE clause to the SQL query.
    fn UPDATE_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self;
    /// Adds a UPDATE clause to the SQL query.
    fn UPDATE<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Iterable;
    fn WHERE_AND<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Iterable;
    fn WHERE_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn WHERE(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self;
    fn to_string(&self) -> io::Result<String>;
    fn build(&self) -> io::Result<String>;
}

#[derive(Clone, Debug, Default)]
pub struct SqlQueryBuilder {
    query: String,
    table: String,
    error_message: Option<String>,
}

/// Represents a SQL Query Builder.
#[allow(non_snake_case)]
impl QueryBuilder for SqlQueryBuilder {
    fn new() -> Self {
        Self {
            query: String::new(),
            table: String::new(),
            error_message: None,
        }
    }

    fn table(table: &str) -> Self {
        let table = sql_injection_prevention(&table.to_string());
        Self {
            query: String::new(),
            table,
            error_message: None,
        }
    }

    fn AND_NOT(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(&operand.to_string());
        let operator = sql_injection_prevention(&operator.to_string());
        let result = sql_injection_prevention(&result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("AND NOT ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn AND(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(&operand.to_string());
        let operator = sql_injection_prevention(&operator.to_string());
        let result = sql_injection_prevention(&result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("AND ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn FROM(&mut self, table: &str) -> &mut Self {
        let table = sql_injection_prevention(&table.to_string());
        self.query.push_str("FROM ");
        self.query.push_str(&table);
        self.query.push_str(" ");
        self
    }

    fn INSERT_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self {
        let mut columns = String::new();
        let mut values = String::new();
        for (column, value) in columns_and_values {
            columns.push_str(&sql_injection_prevention(&column.to_string()));
            columns.push_str(", ");
            values.push_str("'");
            values.push_str(&sql_injection_prevention(&value.to_string()));
            values.push_str("', ");
        }
        // let table = sql_injection_prevention(&table.to_string());
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
        T: Iterable,
    {
        let iterable: HashMap<String, String> = iterate_struct(columns);

        let mut columns = String::new();
        let mut values = String::new();

        for (column, value) in &iterable {
            columns.push_str(&sql_injection_prevention(&column.to_string()));
            columns.push_str(", ");
            values.push_str("'");
            values.push_str(&sql_injection_prevention(&value.to_string()));
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

    fn JOIN(&mut self, table: &str, column1: &str, operator: &str, column2: &str) -> &mut Self {
        let table = sql_injection_prevention(table);
        let column1 = sql_injection_prevention(column1);
        let column2 = sql_injection_prevention(column2);
        let join_clause = format!("JOIN {} ON {} {} {}", table, column1, operator, column2);
        self.query.push_str(&join_clause);
        self.query.push(' ');
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
        let operand = sql_injection_prevention(&operand.to_string());
        let operator = sql_injection_prevention(&operator.to_string());
        let result = sql_injection_prevention(&result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("OR NOT ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn OR(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(&operand.to_string());
        let operator = sql_injection_prevention(&operator.to_string());
        let result = sql_injection_prevention(&result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("OR ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn ORDER_BY(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(&column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("ORDER BY ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn RETURNING(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(&column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("RETURNING ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn SELECT_DISTINCT(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(&column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("SELECT DISTINCT ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn SELECT(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(&column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("SELECT ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn SET(&mut self, columns: &[&str]) -> &mut Self {
        let columns = columns
            .iter()
            .map(|column| sql_injection_prevention(&column.to_string()))
            .collect::<Vec<String>>();
        self.query.push_str("SET ");
        self.query.push_str(&columns.join(", "));
        self.query.push_str(" ");
        self
    }

    fn UPDATE_AS_SLICE(&mut self, columns_and_values: &[(&str, &str)]) -> &mut Self {
        let mut sets = String::new();
        for (column, value) in columns_and_values {
            sets.push_str(&sql_injection_prevention(&column.to_string()));
            sets.push_str(" = ");
            sets.push_str("'");
            sets.push_str(&sql_injection_prevention(&value.to_string()));
            sets.push_str("', ");
        }
        if sets.len() <= 2 {
            panic!("No columns and values provided");
        }
        // let table = sql_injection_prevention(&table.to_string());
        self.query.push_str("UPDATE ");
        self.query.push_str(&self.table);
        self.query.push_str(" SET ");
        self.query.push_str(&sets[..sets.len() - 2]);
        self.query.push_str(" ");
        self
    }

    fn UPDATE<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Iterable,
    {
        let iterable: HashMap<String, String> = iterate_struct(columns);

        let mut values: Vec<(&str, &str)> = Vec::new();
        for item in &iterable {
            if !(item.1 == "" || item.1 == "null") {
                values.push((&item.0, &item.1));
            }
        }

        let mut sets = String::new();
        for (column, value) in values {
            sets.push_str(&sql_injection_prevention(&column.to_string()));
            sets.push_str(" = ");
            sets.push_str("'");
            sets.push_str(&sql_injection_prevention(&value.to_string()));
            sets.push_str("', ");
        }
        if sets.len() <= 2 {
            panic!("No columns and values provided");
        }
        // let table = sql_injection_prevention(&table.to_string());
        self.query.push_str("UPDATE ");
        self.query.push_str(&self.table);
        self.query.push_str(" SET ");
        self.query.push_str(&sets[..sets.len() - 2]);
        self.query.push_str(" ");
        self
    }

    fn WHERE_AND<T>(&mut self, columns: &T) -> &mut Self
    where
        T: Iterable,
    {
        let iterable: HashMap<String, String> = iterate_struct(columns);

        if iterable.len() == 0 {
            return self;
        }

        let mut columns = String::new();

        for (column, value) in &iterable {
            columns.push_str(&sql_injection_prevention(&column.to_string()));
            columns.push_str(" = '");
            columns.push_str(&sql_injection_prevention(&value.to_string()));
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
        let operand = sql_injection_prevention(&operand.to_string());
        let operator = sql_injection_prevention(&operator.to_string());
        let result = sql_injection_prevention(&result.to_string());
        let predicate = format!("{} {} '{}'", operand, operator, result);
        self.query.push_str("WHERE NOT ");
        self.query.push_str(&predicate);
        self.query.push_str(" ");
        self
    }

    fn WHERE(&mut self, operand: &str, operator: &str, result: &str) -> &mut Self {
        let operand = sql_injection_prevention(&operand.to_string());
        let operator = sql_injection_prevention(&operator.to_string());
        let result = sql_injection_prevention(&result.to_string());
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

        #[derive(Iterable)]
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
        #[derive(Iterable)]
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
        #[derive(Iterable)]
        struct User {
            user_name: Option<String>,
            id: Option<String>,
        }

        let user = User {
            user_name: Some("John".to_string()),
            id: Some("1".to_string())
        };

        let query = SqlQueryBuilder::new()
            .SELECT(&["*"])
            .WHERE_AND(&user)
            .build()
            .unwrap();

        assert_eq!(query, "SELECT * WHERE user_name = 'John' AND id = '1';");
    }
}