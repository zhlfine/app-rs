use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn get<T, F>(v: T, key: &str, f: F) -> Option<T>
where
    F: Fn(T, &str) -> Option<T>,
{
    if key.is_empty() {
        return None;
    }

    let vecs: Vec<&str> = key.splitn(2, '.').collect();
    let v2 = f(v, vecs[0]);
    if vecs.len() == 1 {
        v2
    } else {
        v2.map_or_else(|| None, |v3| get(v3, vecs[1], f))
    }
}

#[derive(Debug)]
pub enum Config {
    JSON(serde_json::Value),
    TOML(toml::value::Value),
}

impl Config {

    pub fn get(&self, key: &str) -> Option<Config> {
        match self {
            Config::JSON(v) => get(v, key, |v2,s|v2.get(s)).map(|v3| Config::JSON(v3.clone())),
            Config::TOML(v) => get(v, key, |v2,s|v2.get(s)).map(|v3| Config::TOML(v3.clone())),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        match self {
            Config::JSON(v) => v.as_object().map_or_else(|| vec![], |m| m.keys().cloned().collect()),
            Config::TOML(v) => v.as_table().map_or_else(|| vec![], |m| m.keys().cloned().collect()),
        }
    }

    pub fn str(&self, key: &str) -> Option<String> {
        self.get(key).map_or_else(||None, |s| s._as_str())
    }

    pub fn str_must(&self, key: &str) -> String {
        self.str(key).expect(&format!("failed to get config {} (str)", key))
    }

    pub fn str_default(&self, key: &str, def: &str) -> String {
        self.str(key).unwrap_or(def.to_owned())
    }

    pub fn bool(&self, key: &str) -> Option<bool> {
        self.get(key).map_or_else(|| None, |s| s._as_bool())
    }

    pub fn bool_must(&self, key: &str) -> bool {
        self.bool(key).expect(&format!("failed to get config {} (bool)", key))
    }
    
    pub fn bool_default(&self, key: &str, def: bool) -> bool {
        self.bool(key).unwrap_or(def)
    }

    pub fn i64(&self, key: &str) -> Option<i64> {
        self.get(key).map_or_else(|| None, |s| s._as_i64())
    }

    pub fn i64_must(&self, key: &str) -> i64 {
        self.i64(key).expect(&format!("failed to get config {} (i64)", key))
    }

    pub fn i64_default(&self, key: &str, def: i64) -> i64 {
        self.i64(key).unwrap_or(def)
    }

    pub fn f64(&self, key: &str) -> Option<f64> {
        self.get(key).map_or_else(|| None, |s| s._as_f64())
    }

    pub fn f64_must(&self, key: &str) -> f64 {
        self.f64(key).expect(&format!("failed to get config {} (f64)", key))
    }

    pub fn f64_default(&self, key: &str, def: f64) -> f64 {
        self.f64(key).unwrap_or(def)
    }

    fn _as_str(&self) -> Option<String> {
        match self {
            Config::JSON(v) => match v {
                serde_json::Value::String(s) => Some(s.to_owned()),
                serde_json::Value::Number(s) => Some(format!("{}", s)),
                serde_json::Value::Bool(s) => Some(format!("{}", s)),
                _ => None,
            },
            Config::TOML(v) => match v {
                toml::value::Value::String(s) => Some(s.to_owned()),
                toml::value::Value::Integer(s) => Some(format!("{}", s)),
                toml::value::Value::Float(s) => Some(format!("{}", s)),
                toml::value::Value::Boolean(s) => Some(format!("{}", s)),
                toml::value::Value::Datetime(s) => Some(format!("{}", &s)),
                _ => None,
            },
        }.map_or_else(|| None, |v| if v.is_empty() {None} else {Some(v)})
    }

    fn _as_bool(&self) -> Option<bool> {
        match self {
            Config::JSON(v) => match v {
                serde_json::Value::String(s) => match s.to_lowercase().as_str() {
                    "true" | "yes" | "t" | "y" => Some(true),
                    "false" | "no" | "f" | "n" => Some(false),
                    _ => None,
                },
                serde_json::Value::Bool(s) => Some(*s),
                _ => None,
            },
            Config::TOML(v) => match v {
                toml::value::Value::String(s) => match s.to_lowercase().as_str() {
                    "true" | "yes" | "t" | "y" => Some(true),
                    "false" | "no" | "f" | "n" => Some(false),
                    _ => None,
                },
                toml::value::Value::Boolean(s) => Some(*s),
                _ => None,
            },
        }

    }

    fn _as_i64(&self) -> Option<i64> {
        match self {
            Config::JSON(v) => match v {
                serde_json::Value::String(s) => s.parse::<i64>().ok(),
                serde_json::Value::Number(s) => s.as_i64(),
                _ => None,
            },
            Config::TOML(v) => match v {
                toml::value::Value::String(s) => s.parse::<i64>().ok(),
                toml::value::Value::Integer(s) => Some(*s),
                toml::value::Value::Float(s) => Some(*s as i64),
                _ => None,
            },
        }
    }

    fn _as_f64(&self) -> Option<f64> {
        match self {
            Config::JSON(v) => match v {
                serde_json::Value::String(s) => s.parse::<f64>().ok(),
                serde_json::Value::Number(s) => s.as_f64(),
                _ => None,
            },
            Config::TOML(v) => match v {
                toml::value::Value::String(s) => s.parse::<f64>().ok(),
                toml::value::Value::Integer(s) => Some(*s as f64),
                toml::value::Value::Float(s) => Some(*s),
                _ => None,
            },
        }
    }

}

pub fn load(var: &str, def: &str) -> Config {
    let f = match env::var(var) {
        Ok(s) => s,
        Err(_) => String::from(def),
    };

    let p = if !f.starts_with("/") {
        match env::current_exe() {
            Ok(p) => p.parent().unwrap().join(&f).to_owned(),
            Err(_) => PathBuf::from(&f),
        }
    } else {
        PathBuf::from(&f)
    };

    let mut s = String::new();
    File::open(&p).expect(&format!("failed to open {:?}", &p)).read_to_string(&mut s).unwrap();
    let s = shellexpand::env_with_context_no_errors(&s, _subst).to_owned().to_string();

    let ext = p.extension().unwrap().to_str();
    match ext {
        Some("toml") => Config::TOML(toml::from_str::<toml::value::Value>(&s).unwrap()),
        Some("json") => Config::JSON(serde_json::from_str::<serde_json::Value>(&s).unwrap()),
        Some(_) => panic!("unsupported config {:?}", p),
        None    => panic!("unsupported config {:?}", p),
    }
}

fn _subst(s: &str) -> Option<String> {
    match env::var(s).ok() {
        Some(v) => Some(v),
        None => Some("".to_string()),
        // None => {
        //     match s {
        //         "ABC" => Some("123".to_string()),
        //         _ => Some("".to_string()),
        //     }
        // }
    }
}
