use log::LevelFilter;
use super::config::{self, Config};
use super::db::DbContext;

fn init_logging(config: &Config) {
    let level = config.str_default("log.level", "DEBUG");
    let level_filter = match level.as_str() {
        "TRACE" => LevelFilter::Trace,
        "DEBUG" => LevelFilter::Debug,
        "INFO" => LevelFilter::Info,
        "WARN" => LevelFilter::Warn,
        "ERROR" => LevelFilter::Error,
        "OFF" => LevelFilter::Off,
        _ => panic!("unsupported log level {:?}", level),
    };
    env_logger::Builder::new().default_format_module_path(false).filter_level(level_filter).init();
}

pub fn init() -> AppContext {
    let conf = config::load("APP_CONF", "app_conf.toml");
    let sqls = config::load("SQL_CONF", "sql_conf.toml");

    let db = DbContext::new(&sqls);
    init_logging(&conf);

    AppContext {
        config: conf,
        db: db,
    }
}

pub struct AppContext {
    config: Config,
    db: DbContext,
}

impl AppContext {
    
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db(&self) -> &DbContext {
        &self.db
    }

}