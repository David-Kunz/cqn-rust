mod entities;

#[derive(Debug)]
struct SELECT {
    from: String,
    columns: Vec<String>,
    filter: Vec<String>,
}

impl SELECT {
    pub fn from(entity: &str) -> SELECT {
        SELECT {
            from: entity.to_string(),
            columns: vec![],
            filter: vec![],
        }
    }
    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        let cols: Vec<String> = columns.iter().map(|col| col.to_string()).collect();
        self.columns.extend(cols);
        self
    }

    pub fn filter(mut self, filter: Vec<&str>) -> Self {
        let filter: Vec<String> = filter.iter().map(|col| col.to_string()).collect();
        self.filter.extend(filter);
        self
    }
}

trait CQN {
    fn to_sql(&self) -> String;
}

impl CQN for SELECT {
    fn to_sql(&self) -> String {
        let mut res = match &self.columns.len() > &0 {
            true => format!("SELECT {} FROM {}", &self.columns.join(","), &self.from),
            false => format!("SELECT * FROM {}", &self.from),
        };
        if &self.filter.len() > &0 {
            res = format!("{}{}", res, format!("\n  WHERE {}", &self.filter.join(" ")));
        }
        res
    }
}

fn main() {
    let select = SELECT::from("example_entity").filter(vec![
        "(", "a", ">", "2", "and", "b", "<", "9", ")", "or", "c", "<", "4",
    ]);
    println!("{}", select.to_sql());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_with_col_to_sql() {
        let select = SELECT::from("example_entity").columns(vec!["col1", "col2"]);
        assert_eq!(select.to_sql(), "SELECT col1,col2 FROM example_entity")
    }

    #[test]
    fn select_without_col_to_sql() {
        let select = SELECT::from("example_entity");
        assert_eq!(select.to_sql(), "SELECT * FROM example_entity")
    }

    #[test]
    fn select_with_filter_to_sql() {
        let select = SELECT::from("example_entity").filter(vec![
            "(", "a", ">", "2", "and", "b", "<", "9", ")", "or", "c", "<", "4",
        ]);
        assert_eq!(
            select.to_sql(),
            "SELECT * FROM example_entity\n  WHERE ( a > 2 and b < 9 ) or c < 4"
        )
    }
}
