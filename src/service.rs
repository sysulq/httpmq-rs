use axum::{extract::Extension, extract::Query, http::StatusCode, response::IntoResponse};
use once_cell::sync::OnceCell;
use rocksdb::DB;
use serde::Deserialize;
use std::{
    borrow::Cow,
    str,
    sync::{Arc, RwLock},
};
use tower::BoxError;
use tracing::debug;

pub static DEFAULT_MAX_QUEUE_CELL: OnceCell<i32> = OnceCell::new();

// httpmq read metadata api
// retrieve from leveldb
// name.maxqueue - maxqueue
// name.putpos - putpos
// name.getpos - getpos
fn httpmq_read_metadata(db: &rocksdb::DB, name: &String) -> Option<Vec<i32>> {
    let mut result: Vec<_> = db
        .multi_get(vec![
            name.to_string() + ".maxqueue",
            name.to_string() + ".putpos",
            name.to_string() + ".getpos",
        ])
        .iter()
        .map(|x| match x {
            Ok(Some(xx)) => str::from_utf8(xx).unwrap().parse::<i32>().unwrap(),
            _ => 0,
        })
        .collect();

    debug!("result {:?}", result);
    if result[0] == 0 {
        result[0] = *DEFAULT_MAX_QUEUE_CELL.get().unwrap();
    }
    Some(result)
}

fn httpmq_now_getpos(db: &rocksdb::DB, name: &String) -> Option<i32> {
    let metadata = httpmq_read_metadata(db, name);
    let maxqueue = metadata.as_ref()?[0];
    let putpos = metadata.as_ref()?[1];
    let mut getpos = metadata.as_ref()?[2];

    if getpos == 0 && putpos > 0 {
        getpos = 1 // first get operation, set getpos 1
    } else if getpos < putpos {
        getpos += 1 // 1nd lap, increase getpos
    } else if getpos > putpos && getpos < maxqueue {
        getpos += 1 // 2nd lap
    } else if getpos > putpos && getpos == maxqueue {
        getpos = 1 // 2nd first operation, set getpos 1
    } else {
        return Some(0); // all data in queue has been get
    }

    debug!("getpos {} {:?}", getpos, metadata);

    db.put(name.to_string() + ".getpos", getpos.to_string())
        .ok()?;
    Some(getpos)
}

fn httpmq_now_putpos(db: &rocksdb::DB, name: &String) -> Option<i32> {
    let metadata = httpmq_read_metadata(db, name);
    let maxqueue = metadata.as_ref()?[0];
    let mut putpos = metadata.as_ref()?[1];
    let getpos = metadata.as_ref()?[2];

    let newpos;

    putpos += 1; // increase put queue pos
    if putpos == getpos {
        // queue is full
        return Some(0); // return 0 to reject put operation
    } else if getpos <= 1 && putpos > maxqueue {
        // get operation less than 1
        return Some(0); // and queue is full, just reject it
    } else if putpos > maxqueue {
        //  2nd lap
        newpos = 1 // reset putpos as 1 and write to leveldb
    } else {
        // 1nd lap, convert int to string and write to leveldb
        newpos = putpos;
    }

    debug!("newpos {} {:?}", newpos, metadata);

    db.put(name.to_string() + ".putpos", newpos.to_string())
        .unwrap();

    Some(newpos)
}

pub type SharedState = Arc<RwLock<State>>;

pub struct State {
    database: rocksdb::DB,
}

impl State {
    pub fn new() -> State {
        let db = DB::open_default("path").unwrap();
        State { database: db }
    }
}

async fn kv_get(
    Query(args): Query<KVSet>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    let db = &state.read().unwrap().database;
    let getpos = httpmq_now_getpos(&db, &args.name).unwrap_or_default();

    debug!("{} {:?}", getpos, args);

    if getpos == 0 {
        Ok(String::from("HTTPMQ_GET_END"))
    } else {
        let queue_name = args.name.to_string() + &getpos.to_string();
        let val = match db.get(queue_name) {
            Ok(Some(obj)) => String::from_utf8(obj.clone()).unwrap_or(String::from("")),
            Ok(None) => String::from("HTTPMQ_GET_NONE"),
            Err(_) => String::from("HTTPMQ_GET_ERROR"),
        };

        Ok(val)
    }
}

#[derive(Deserialize, Debug)]
pub struct KVSet {
    opt: String,
    name: String,
    data: Option<String>,
    // pos: Option<i32>,
    num: Option<i32>,
}

async fn kv_maxqueue(
    Query(args): Query<KVSet>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    let num = args.num.unwrap_or(0);
    if num > 0 && num <= *DEFAULT_MAX_QUEUE_CELL.get().unwrap() {
        let db = &state.read().unwrap().database;
        db.put(args.name.to_string() + ".maxqueue", num.to_string())
            .unwrap();
        Ok(String::from("HTTPMQ_MAXQUEUE_OK"))
    } else {
        Ok(String::from("HTTPMQ_MAXQUEUE_CANCLE"))
    }
}

async fn kv_set(
    Query(args): Query<KVSet>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    let db = &state.read().unwrap().database;

    let putpos = httpmq_now_putpos(&db, &args.name).unwrap_or_default();

    debug!("{} {:?}", putpos, args);

    if putpos > 0 {
        let queue_name = args.name.to_string() + &putpos.to_string();
        let data = args.data.unwrap_or("".to_string());
        if data.len() > 0 {
            db.put(queue_name, data).unwrap();
            return Ok(String::from("HTTPMQ_PUT_OK"));
        }
        Ok(String::from("HTTPMQ_PUT_NO_DATA"))
    } else {
        Ok(String::from("HTTPMQ_PUT_END"))
    }
}

async fn kv_status(
    Query(args): Query<KVSet>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    let db = &state.read().unwrap().database;
    let metadata = httpmq_read_metadata(db, &args.name).unwrap_or(vec![0, 0, 0]);
    let maxqueue = metadata[0];
    let putpos = metadata[1];
    let getpos = metadata[2];

    let mut ungetnum = 0;
    let mut put_times = "";
    let mut get_times = "";
    if putpos >= getpos {
        ungetnum = (putpos - getpos).abs();
        put_times = "1st lap";
        get_times = "1st lap";
    } else if putpos < getpos {
        ungetnum = (maxqueue + putpos - getpos).abs();
        put_times = "2st lap";
        get_times = "1st lap";
    }

    let buf = format!(
        "HTTP Simple Queue Service
------------------------------
Queue Name: {}
Maximum number of queues: {}
Put position of queue ({}): {}
Get position of queue ({}): {}
Number of unread queue: {}
",
        args.name.to_string(),
        maxqueue,
        put_times,
        putpos,
        get_times,
        getpos,
        ungetnum
    );

    Ok(buf)
}

async fn kv_reset(
    Query(args): Query<KVSet>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    let db = &state.read().unwrap().database;
    db.put(
        args.name.to_string() + ".maxqueue",
        DEFAULT_MAX_QUEUE_CELL.get().unwrap().to_string(),
    )
    .unwrap();
    db.put(args.name.to_string() + ".putpos", "0").unwrap();
    db.put(args.name.to_string() + ".getpos", "0").unwrap();

    Ok(String::from("HTTPMQ_RESET_OK"))
}

pub async fn process(
    Query(args): Query<KVSet>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    let res = match &args.opt[..] {
        "get" => kv_get(Query(args), Extension(state)).await,
        "put" => kv_set(Query(args), Extension(state)).await,
        "status" => kv_status(Query(args), Extension(state)).await,
        "reset" => kv_reset(Query(args), Extension(state)).await,
        "maxqueue" => kv_maxqueue(Query(args), Extension(state)).await,
        _ => Ok(String::from("invalid opt")),
    };

    return res;
}

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}
