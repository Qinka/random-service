use actix_web::{web, App, HttpResponse, HttpServer, Result, HttpRequest};
use crate::error::{R, Resp, ErrorResponse, to_ErrorResponse};
use crate::state::{SvrState, AuthState, State};
use r2d2_mongodb::mongodb::{doc, Bson, bson};
use r2d2_mongodb::mongodb::coll::Collection;
use r2d2_mongodb::mongodb::db::ThreadedDatabase;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::offset::Utc;
use log::{info,error};
use std::collections::HashSet;


#[derive(Deserialize)]
pub struct GetTokenForm {
    expire: i64, //seconds
}

#[derive(Serialize)]
pub struct GetTokenResp {
    token: String,
    user_agent: String,
    auths: Vec<String>,
    expire: i64
}

fn generate_token(coll: Collection) -> Result<String, ErrorResponse> {
    let uuid = Uuid::new_v4().hyphenated().to_string();
    let rest = coll.find_one(Some(doc!{"token": &uuid}), None).map_err(to_ErrorResponse)?;

    if let Some(_) = rest  {
        generate_token(coll)
    } else {
        Ok(uuid)
    }
}

pub async fn get_token(form: Option<web::Json<GetTokenForm>>, state: State, req: HttpRequest) -> Resp {
    let db = state.auth.mongo.clone().get().map_err(to_ErrorResponse)?;
    let token_coll = db.collection("token");
    let uuid = generate_token(token_coll)?;
    let ua   = req.headers().get("User-Agent")
        .map(|h| h.to_str().map(String::from).map_err(to_ErrorResponse))
        .unwrap_or(Ok("".to_owned()))?;
    let gp_coll = db.collection("gp");
    let anonymous_auths = gp_coll.find_one(Some(doc!{"name": "anonymous"}), None).map_err(to_ErrorResponse)?;
    let auths = match anonymous_auths {
        None => Vec::new(),
        Some(d) => {
            let aus = d.get_array("auths").map_err(to_ErrorResponse)?;
            aus.iter().map(|a| String::from(a.as_str().unwrap())).collect()
        }
    };
    let tk_coll = db.collection("token");
    let expire  = form.map(|f| f.expire).unwrap_or(1800) + Utc::now().timestamp();
    let auths_bson: Vec<_> = auths.iter().map(|a| Bson::from(a)).collect();
    tk_coll.insert_one(doc!{
            "token": &uuid, "user": "anonymous", "agent": &ua,
            "right": auths_bson, "expire": Bson::TimeStamp(expire)
        }, None).map_err(to_ErrorResponse)?;

    R::ok(GetTokenResp{token: uuid, user_agent: ua, auths: auths, expire: expire}).to_json_result()
}

#[derive(Deserialize)]
pub struct PutTokenForm {
    user: String,
    pass: String,
    token: String,
    item: Vec<String>,
    expire: Option<i64>,
}

pub async fn put_token(form: web::Json<PutTokenForm>, state: State, req: HttpRequest) -> Resp {
    let db = state.auth.mongo.clone().get().map_err(to_ErrorResponse)?;
    let token_coll = db.collection("token");
    let token = token_coll.find_one(Some(doc!{"token": &form.token}), None)
        .map_err(to_ErrorResponse)?
        .ok_or(ErrorResponse::InvailedToken{token: form.token.clone()})?;
    if token.get_time_stamp("expire").ok().map_or(true, |i| i > Utc::now().timestamp()) {
        let user_coll  = db.collection("user");
        let user = user_coll.find_one(Some(doc!{"user": &form.user, "pass": &form.pass}), None)
            .map_err(to_ErrorResponse)?.ok_or(ErrorResponse::LoginFailed)?;
        let user_gps = user.get_array("group").map_err(to_ErrorResponse)?
            .iter().map(|b| b.clone()).collect::<Vec<_>>();
        let user_len = user_gps.len();
        let gp_coll = db.collection("gp");

        let auth_set = gp_coll.find(Some(doc!{"name": {"$in": user_gps}}), None)
            .map_err(to_ErrorResponse)?
            .try_fold(HashSet::with_capacity(user_len), |mut hash, r| {
                r.map_err(to_ErrorResponse)?.get_array("auths").map_err(to_ErrorResponse)?
                    .iter().try_for_each(|a| a.as_str().map(|s| {hash.insert(String::from(s)); ()})
                    .ok_or(ErrorResponse::InternalError{reason: "Right not string".to_owned()}))?;
                Ok(hash)
            })?;
        let expire = form.expire.map_or(Bson::Null, |ex| Bson::TimeStamp(ex));
        token_coll.update_one(doc!{"token": &form.token}, doc!{"$set":
                    {"expire": expire, "user": &form.user,
                        "right": auth_set.iter().map(|a| Bson::from(a)).collect::<Vec<_>>()}
                }, None)
            .map_err(to_ErrorResponse)?;
        R::ok(()).to_json_result()
    } else {
        Err(ErrorResponse::InvailedToken{token: form.token.clone()})
    }
}


#[derive(Deserialize)]
pub struct DelTokenForm {
    user: String,
    token: String,
}

pub async fn del_token(form: web::Json<DelTokenForm>, state: State) -> Resp {
    let db = state.auth.mongo.clone().get().map_err(to_ErrorResponse)?;
    let token_coll = db.collection("token");
    let dr = token_coll.delete_many(doc!{"user": &form.user, "token": &form.token}, None)
        .map_err(to_ErrorResponse)?;
    if dr.deleted_count == 0 {
        Err(ErrorResponse::InvailedToken{token: form.token.clone()})
    } else {
        R::ok(()).to_json_result()
    }
}

pub fn test_auth(db: Collection, token: &String, right: &String) -> Result<bool, ErrorResponse> {
    let rest = db.find_one(Some(doc!{"token": token}), None).map_err(to_ErrorResponse)?;
    let user = rest.ok_or(ErrorResponse::InvailedToken{token: token.clone()})?;
    if user.get_time_stamp("expire").ok().map_or(false, |i| i < Utc::now().timestamp()) {
        Err(ErrorResponse::InvailedToken{token: token.clone()})
    } else {
        let auths = user.get_array("right").map_err(to_ErrorResponse)?
            .iter().fold(false, |i, r| i || r.as_str().map(|ri| ri == right).unwrap_or(false));
        if auths {
            Ok(true)
        } else {
            Err(ErrorResponse::NoAuthorization)
        }
    }
}


#[derive(Deserialize)]
pub struct GetTokenTestForm {
    token: String,
    right: String,
}

pub async fn get_token_test(form: web::Json<GetTokenTestForm>, state: State) -> Resp {
    let db = state.auth.mongo.clone().get().map_err(to_ErrorResponse)?;
    let token_coll = db.collection("token");
    if test_auth(token_coll, &form.token, &form.right)? {
        R::ok(()).to_json_result()
    } else {
        Err(ErrorResponse::NoAuthorization)
    }
}