use crate::parser::join::FromType;
use sqlparser::ast::Statement;
use std::collections::HashMap;
use crate::parser::condition::Condition;
use crate::parser::utils::parse_sql;
use crate::system::utils::custom_strip;

#[derive(Debug)]
pub struct UpdateQuery {
    pub tb_name: String,
    pub assignments: HashMap<String, String>,
    pub condition: Option<Condition>,
}

impl UpdateQuery {
    pub fn format_stat(statement: Statement) -> UpdateQuery {
        let mut tb_name: String = "".to_string();
        let mut assignments_data: HashMap<String, String> = HashMap::new();
        let mut condition_data: Option<Condition> = None;
        if let Statement::Update {
            table,
            assignments,
            selection,
            ..
        } = statement
        {
            let from = FromType::new(vec![table]).first().unwrap().to_owned();
            if let FromType::String { tb } = from {
                tb_name = tb;
            }
            assignments_data = assignments
                .iter()
                .map(|assign| {
                    (
                        assign.id.first().unwrap().to_owned().value.to_string(),
                        custom_strip(custom_strip(assign.value.to_string().as_str(), "\'"), "\"")
                            .to_string(),
                    )
                })
                .collect::<HashMap<String, String>>();
            condition_data = Option::from(Condition::from_expr(&selection.unwrap()));
        }
        UpdateQuery {
            tb_name,
            assignments: assignments_data,
            condition: condition_data,
        }
    }
}

#[test]
pub fn test_update_query() {
    let sql = "UPDATE users SET password = 'new_password', email = \"new_email@example.com\" WHERE id = 1;";
    let state = parse_sql(sql);
    println!("{:?}", state);
    UpdateQuery::format_stat(state);
}
