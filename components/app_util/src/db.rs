use std::collections::HashMap;
use regex::{Regex, Captures};
use super::config::Config;


pub struct DbContext {
    sql_defs: HashMap<String, SqlDef>,
}

impl DbContext {

    pub fn new(config: &Config) -> DbContext {
        let mut map = HashMap::<String, SqlDef>::new();
        for key in config.keys().iter() {
            let sql_conf = config.get(key).unwrap();
            let sql = SqlDef::new(key, &sql_conf);
            map.insert(key.to_owned(), sql);
        }
        DbContext {
            sql_defs: map,
        }
    }

    pub fn query<R: SqlRenderer>(&self, name: &str, r: R) {
        // let sql = self.sql(name, r);
    }

    pub fn exec<R: SqlRenderer>(&self, name: &str, r: R) {
        // let sql = self.sql(name, r);
        
    }

    pub fn sql<R: SqlRenderer>(&self, name: &str, r: R) -> String {
        let sql_def = self.sql_defs.get(name).expect(&format!("sql {} not found", name));
        r.render(sql_def)
    }

}

#[derive(Debug)]
pub enum SqlType {
    QueryList,
}

impl SqlType {

    pub fn new(typ: &str) -> SqlType {
        match typ {
            "query_list" => SqlType::QueryList,
            _ => panic!("unsupported sql type {:?}", typ),
        }
    }

}

#[derive(Debug)]
pub struct SqlDef {
    pub name: String,
    pub typ: SqlType,
    pub sql: String,
}

impl SqlDef {

    pub fn new(name: &str, config: &Config) -> SqlDef {
        let typ = config.str_default("type", "query_list");
        let sql = config.str_must("sql");
        
        SqlDef {
            name: name.to_owned(),
            typ: SqlType::new(&typ),
            sql: sql,
        }
    }

}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\{\{([_0-9a-zA-Z]+)\}\}").unwrap();
}

pub trait SqlRenderer {
    fn render(&self, sql_def: &SqlDef) -> String;
}

impl<F> SqlRenderer for F
where 
    F: Fn(&str) -> String
{
    fn render(&self, sql_def: &SqlDef) -> String {
        RE.replace_all(&sql_def.sql, |caps: &Captures| {
            caps.get(1).map(|m| self(m.as_str())).unwrap_or(String::from(""))
        }).to_owned().to_string()
    }
}

impl SqlRenderer for Vec<&str> {
    fn render(&self, sql_def: &SqlDef) -> String {
        let mut it = self.iter();
        RE.replace_all(&sql_def.sql, |_: &Captures| {
            it.next().expect(&format!("sql {} args error", sql_def.name)).to_owned().to_owned()
        }).to_owned().to_string()
    }
}

impl SqlRenderer for Vec<(&str, &str)> {
    fn render(&self, sql_def: &SqlDef) -> String {
        RE.replace_all(&sql_def.sql, |caps: &Captures| {
            match caps.get(1) {
                Some(m) => {
                    let p = m.as_str();
                    for (k, v) in self.into_iter() {
                        if p == *k {
                            return *v;
                        }
                    }
                    panic!("sql {} args error", sql_def.name);
                },
                None => unreachable!("unknown sql param match"),
            }
        }).to_owned().to_string()
    }
}


