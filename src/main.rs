use std::env;
use log::debug;
use app_util::context;

fn main() {

    env::set_var("APP_CONF", "../../etc/app_conf.toml");
    env::set_var("SQL_CONF", "../../etc/sql_conf.toml");
 
    let ctx = context::init();

    let v2 = ctx.config().str_must("version");
    debug!("config: {:?}", v2);

    let sql = ctx.db().sql("fullsync_device", |s: &str| format!("5{}5", s));
    debug!("sql1: {}", sql);

    let sql = ctx.db().sql("fullsync_device", vec!("3", "4"));
    debug!("sql2: {}", sql);

    let sql = ctx.db().sql("fullsync_device", vec!(("b", "6"),("a", "5")));
    debug!("sql3: {}", sql);

}