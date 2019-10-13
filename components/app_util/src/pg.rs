use log::info;
use postgres::{Connection, TlsMode};

pub fn test2() {
    let uri = "postgres://postgres@cdc-cloud09:5432/sxa_aqa";
    let conn = Connection::connect(uri, TlsMode::None).unwrap();

    let rows = conn.query("SELECT subscriber_id,subscriber_location_id,name FROM cloud_subscribers limit 10", &[]).unwrap();
    for row in &rows {
        let subscriber_id: String = row.get(0);
        let subscriber_location_id: String = row.get(1);
        let name: String = row.get(2);

        info!("subscriber: {}, {}, {}", subscriber_id, subscriber_location_id, name);
    }

}

// pub fn copy(src: &Connection, dst: &Connection) {
    
// }
