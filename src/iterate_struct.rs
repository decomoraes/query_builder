use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use crate::Iterable;

pub fn iterate_struct<T: Iterable>(s: &T) -> HashMap<String, String>
where
    T: Iterable,
{
    s.iter().filter_map(|(key, value)| {
        if let Some(string_opt) = value.downcast_ref::<Option<String>>() {
            if let Some(string) = string_opt.as_deref() {
                return Some((key.to_string(), format!("{}", string)));
            }
        } else if let Some(u8_opt) = value.downcast_ref::<Option<u8>>() {
            if let Some(u8) = u8_opt {
                return Some((key.to_string(), format!("{}", u8)));
            }
        } else if let Some(u16_opt) = value.downcast_ref::<Option<u16>>() {
            if let Some(u16) = u16_opt {
                return Some((key.to_string(), format!("{}", u16)));
            }
        } else if let Some(u32_opt) = value.downcast_ref::<Option<u32>>() {
            if let Some(u32) = u32_opt {
                return Some((key.to_string(), format!("{}", u32)));
            }
        } else if let Some(u64_opt) = value.downcast_ref::<Option<u64>>() {
            if let Some(u64) = u64_opt {
                return Some((key.to_string(), format!("{}", u64)));
            }
        } else if let Some(usize_opt) = value.downcast_ref::<Option<usize>>() {
            if let Some(usize) = usize_opt {
                return Some((key.to_string(), format!("{}", usize)));
            }
        } else if let Some(i8_opt) = value.downcast_ref::<Option<i8>>() {
            if let Some(i8) = i8_opt {
                return Some((key.to_string(), format!("{}", i8)));
            }
        } else if let Some(i16_opt) = value.downcast_ref::<Option<i16>>() {
            if let Some(i16) = i16_opt {
                return Some((key.to_string(), format!("{}", i16)));
            }
        } else if let Some(i32_opt) = value.downcast_ref::<Option<i32>>() {
            if let Some(i32) = i32_opt {
                return Some((key.to_string(), format!("{}", i32)));
            }
        } else if let Some(i64_opt) = value.downcast_ref::<Option<i64>>() {
            if let Some(i64) = i64_opt {
                return Some((key.to_string(), format!("{}", i64)));
            }
        } else if let Some(isize_opt) = value.downcast_ref::<Option<isize>>() {
            if let Some(isize) = isize_opt {
                return Some((key.to_string(), format!("{}", isize)));
            }
        } else if let Some(i32_opt) = value.downcast_ref::<Option<i32>>() {
            if let Some(i32) = i32_opt {
                return Some((key.to_string(), format!("{}", i32)));
            }
        } else if let Some(f32_opt) = value.downcast_ref::<Option<f32>>() {
            if let Some(f32) = f32_opt {
                return Some((key.to_string(), format!("{}", f32)));
            }
        } else if let Some(f64_opt) = value.downcast_ref::<Option<f64>>() {
            if let Some(f64) = f64_opt {
                return Some((key.to_string(), format!("{}", f64)));
            }
        } else if let Some(bool_opt) = value.downcast_ref::<Option<bool>>() {
            if let Some(bool) = bool_opt {
                return Some((key.to_string(), format!("{}", bool)));
            }
        } else if let Some(str_opt) = value.downcast_ref::<Option<&str>>() {
            if let Some(str) = str_opt {
                return Some((key.to_string(), format!("{}", str)));
            }
        } else if let Some(string) = value.downcast_ref::<String>() {
            return Some((key.to_string(), format!("{}", string)));
        } else if let Some(u8) = value.downcast_ref::<u8>() {
            return Some((key.to_string(), format!("{}", u8)));
        } else if let Some(u16) = value.downcast_ref::<u16>() {
            return Some((key.to_string(), format!("{}", u16)));
        } else if let Some(u32) = value.downcast_ref::<u32>() {
            return Some((key.to_string(), format!("{}", u32)));
        } else if let Some(u64) = value.downcast_ref::<u64>() {
            return Some((key.to_string(), format!("{}", u64)));
        } else if let Some(usize) = value.downcast_ref::<usize>() {
            return Some((key.to_string(), format!("{}", usize)));
        } else if let Some(i8) = value.downcast_ref::<i8>() {
            return Some((key.to_string(), format!("{}", i8)));
        } else if let Some(i16) = value.downcast_ref::<i16>() {
            return Some((key.to_string(), format!("{}", i16)));
        } else if let Some(i32) = value.downcast_ref::<i32>() {
            return Some((key.to_string(), format!("{}", i32)));
        } else if let Some(i64) = value.downcast_ref::<i64>() {
            return Some((key.to_string(), format!("{}", i64)));
        } else if let Some(isize) = value.downcast_ref::<isize>() {
            return Some((key.to_string(), format!("{}", isize)));
        } else if let Some(f32) = value.downcast_ref::<f32>() {
            return Some((key.to_string(), format!("{}", f32)));
        } else if let Some(f64) = value.downcast_ref::<f64>() {
            return Some((key.to_string(), format!("{}", f64)));
        } else if let Some(bool) = value.downcast_ref::<bool>() {
            return Some((key.to_string(), format!("{}", bool)));
        } else if let Some(str) = value.downcast_ref::<&str>() {
            return Some((key.to_string(), format!("{}", str)));
        }

        None
    }).collect()

    // let struct_converted_to_string = format!("{:#?}", serde_yaml::to_value(s).unwrap());
    // let fix_booleans = fix_json_to_hash(&struct_converted_to_string);
    // let serde_value = serde_yaml::from_str::<serde_yaml::Value>(&fix_booleans).unwrap();
    // serde_yaml::from_value(serde_value).unwrap()
}


// pub fn iterate_struct<T>(s: &T) -> HashMap<String, String>
// where
//     T: Serialize,
// {
//     let struct_converted_to_string = format!("{:#?}", serde_yaml::to_value(s).unwrap());
//     let fix_booleans = fix_json_to_hash(&struct_converted_to_string);
//     let serde_value = serde_yaml::from_str::<serde_yaml::Value>(&fix_booleans).unwrap();
//     serde_yaml::from_value(serde_value).unwrap()
// }

// fn fix_json_to_hash(json: &str) -> String {
//     let re = Regex::new(
//         r#"(?P<key>("?)(.+?)("?: ))(?P<type>String|Bool|Number)(\("?)(?P<value>.+?)("?\))"#,
//     )
//     .unwrap();
//
//     let mut new_json = "{".to_string();
//
//     for caps in re.captures_iter(json) {
//         if &caps["type"] == "Bool" {
//             if &caps["value"] == "true" {
//                 let line = format!(r#"{} "{}", "#, caps["key"].to_string(), "TRUE".to_string(),);
//                 new_json.push_str(&line);
//             } else {
//                 let line = format!(r#"{} "{}", "#, caps["key"].to_string(), "FALSE".to_string());
//                 new_json.push_str(&line);
//             }
//         } else if &caps["value"] == r#"""# {
//             let line = format!(r#"{} "", "#, caps["key"].to_string(),);
//             new_json.push_str(&line);
//         } else {
//             let line = format!(
//                 r#"{} "{}", "#,
//                 caps["key"].to_string(),
//                 caps["value"].to_string()
//             );
//             new_json.push_str(&line);
//         }
//     }
//
//     new_json = new_json.trim_end_matches(", ").to_string();
//     new_json.push_str("}");
//
//     new_json
// }

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
