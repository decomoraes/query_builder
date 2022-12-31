use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

pub fn iterate_struct<T>(s: &T) -> HashMap<String, String>
where
    T: Serialize,
{
    let struct_converted_to_string = format!("{:#?}", serde_yaml::to_value(s).unwrap());
    let fix_booleans = fix_json_to_hash(&struct_converted_to_string);
    let serde_value = serde_yaml::from_str::<serde_yaml::Value>(&fix_booleans).unwrap();
    serde_yaml::from_value(serde_value).unwrap()
}

fn fix_json_to_hash(json: &str) -> String {
    let re = Regex::new(
        r#"(?P<key>("?)(.+?)("?: ))(?P<type>String|Bool|Number)(\("?)(?P<value>.+?)("?\))"#,
    )
    .unwrap();

    let mut new_json = "{".to_string();

    for caps in re.captures_iter(json) {
        if &caps["type"] == "Bool" {
            if &caps["value"] == "true" {
                let line = format!(r#"{} "{}", "#, caps["key"].to_string(), "TRUE".to_string(),);
                new_json.push_str(&line);
            } else {
                let line = format!(r#"{} "{}", "#, caps["key"].to_string(), "FALSE".to_string());
                new_json.push_str(&line);
            }
        } else if &caps["value"] == r#"""# {
            let line = format!(r#"{} "", "#, caps["key"].to_string(),);
            new_json.push_str(&line);
        } else {
            let line = format!(
                r#"{} "{}", "#,
                caps["key"].to_string(),
                caps["value"].to_string()
            );
            new_json.push_str(&line);
        }
    }

    new_json = new_json.trim_end_matches(", ").to_string();
    new_json.push_str("}");

    new_json
}

// pub fn struct_to_slice<T>(s: &T) -> &[(&str, &str)]
//     where
//         T: Serialize,
// {
//     let iterable: HashMap<String, String> = iterate_struct(&s);

//     let mut values: Vec<(&str, &str)> = Vec::new();

//     for item in &iterable {
//         if !(item.1 == "" || item.1 == "null") {
//             values.push((&item.0, &item.1));
//         }
//     }

//     values.as_slice().as_ref()
// }

// ---- tests::should_insert_and_return_from_struct stdout ----
// thread 'tests::should_insert_and_return_from_struct' panicked at 'called `Result::unwrap()` on an `Err` value: Error("EOF while parsing a value")', src/iterate_struct.rs:10:80

// ---- tests::should_update_users_set_name_equal_john_where_id_equal_1 stdout ----
// thread 'tests::should_update_users_set_name_equal_john_where_id_equal_1' panicked at 'called `Result::unwrap()` on an `Err` value: Error("EOF while parsing a value")', src/iterate_struct.rs:10:80
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
