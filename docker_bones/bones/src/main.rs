use actix_cors::Cors;
use actix_web::HttpRequest;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use cookie::time::Duration;
use cookie::Cookie;
use dotenv_codegen::dotenv;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha384;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::collections::BTreeMap;
//stores db connection
const JWT_KEY: &str = dotenv!("JWT_KEY");
const POSTGRES_CONN_STRING: &str = dotenv!("POSTGRES_CONN_STRING");
pub struct AppState {
    db: PgPool,
}

#[derive(Serialize, Deserialize)]
struct StringBill {
    amount: String,
    date: String,
    img_path: String,
    duedate: String,
    providerid: String,
    billstatusid: String,
}

//bill structure setup
#[derive(Serialize, Deserialize, Clone)]
struct Bill {
    id: i32,
    amount: f64,
    date: String,
    img_path: String,
    duedate: String,
    providerid: i32,
    billstatusid: i32,
    provider_name: String,
    bill_status: String,
}
//converts bill to string...not sure if this is used?
impl ToString for Bill {
    fn to_string(&self) -> String {
        format!("{{\"amount\":\"{}\", \"date\":\"{}\", \"img_path\":\"{}\",\"due_date\":\"{}\", \"providerid\":\"{}\",\"billstatusid\":\"{}\"}}", self.amount, self.date, self.img_path, self.duedate, self.providerid,self.billstatusid)
    }
}
//user structure setup
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: i32,
    username: String,
    role: i32,
}
//Error struct
#[derive(Serialize, Deserialize)]
struct Error {
    error: String,
}
//handles user when they are not logged in....?
#[derive(Serialize, Deserialize)]
struct User_no_auth {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
struct NewUser {
    username: String,
    password: String,
    role: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct Provider {
    id: i32,
    url: String,
    phone: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
struct NewProvider {
    url: String,
    phone: String,
    name: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct BillStatus {
    id: i32,
    status: String,
}
fn input_sanitizer(input: String) -> String {
    let input: String = input.replace(" ", "");
    let input: String = input.replace("\\n", "");
    let input: String = input.replace("*", "");
    let input: String = input.replace("{", "");
    let input: String = input.replace("}", "");
    input
}
fn json_builder<T>(vec: Vec<T>) -> String
where
    T: serde::Serialize,
{
    let mut full_string: String = String::from("[");
    for item in vec {
        if (full_string.len() > 1) {
            full_string = [
                full_string.as_str().to_owned(),
                ",".to_string(),
                serde_json::to_string(&item).unwrap().as_str().to_owned(),
            ]
            .concat();
        } else {
            full_string = [
                full_string.as_str().to_owned(),
                serde_json::to_string(&item).unwrap().as_str().to_owned(),
            ]
            .concat();
        }
    }
    full_string.push_str("]");
    full_string
}
fn cookie_unwrapper(cookie: Option<String>) -> String {
    //unwrapping cookie
    match cookie {
        Some(x) => x,
        None => "default".to_string(),
    }
}

#[post("/b/{id}")]
async fn bill_edit(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(u32,)>,
) -> impl Responder {
    //set an owned string so it can be converted to str "c"
    let user_slug: u32 = path.into_inner().0;
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    //c will become the cookie from the client as a &str, after the cookie is unwrapped
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    //pethaps it might be more effected to decrypt both? who knows
    //this key needs moved to an .env file
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    //get db connection
    let mut conn = data.db.acquire().await.unwrap();
    let int_user_role = found_user_role.parse::<i32>().unwrap();
    //get all bills
    //should probably add some conditions here to check for role typei
    let mut row;
    if (int_user_role > 0) {
        row = sqlx::query(format!("
SELECT DISTINCT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE 
    r.id = {} AND -- Placeholder for role ID
    b.id = {}; -- Placeholder for bill ID
", int_user_role,user_slug).as_str())
        .fetch_all(&mut *conn)
        .await;
    }
    /*else if (found_user_role=="2"){
        row = sqlx::query(format!("SELECT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE r.id = 2;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }  else if (found_user_role=="3"){
        row = sqlx::query(format!("SELECT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE r.id = 3;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }*/
    else {
        row = sqlx::query(format!("SELECT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE r.id = 0;").as_str())
        .fetch_all(&mut *conn)
        .await;
    }
    //sets up vector to store bills in
    let mut vec = vec![];
    //unwrap row
    match row {
        Ok(row) => {
            for item in row {
                //could we just instantiate object here? probably
                let bill_item = Bill {
                    id: item.get("id"),
                    amount: item.get("amount"),
                    date: item.get("date"),
                    img_path: item.get("img_path"),
                    duedate: item.get("duedate"),
                    providerid: item.get("providerid"),
                    billstatusid: item.get("billstatusid"),
                    provider_name: item.get("provider_name"),
                    bill_status: item.get("bill_status"),
                };
                vec.push(bill_item);
            }
        }
        Err(e) => {
            //need to get error handling here
            panic!("error finding row")
        }
    }
    let full_string = json_builder(vec.clone());
    HttpResponse::Ok().body(full_string)
}

//end point to get all bills user has access to on main screen
#[post("/b")]
async fn b(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());

    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    //pethaps it might be more effected to decrypt both? who knows
    //this key needs moved to an .env file
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    //get db connection
    let mut conn = data.db.acquire().await.unwrap();
    let int_user_role = found_user_role.parse::<i32>().unwrap();
    //get all bills
    //should probably add some conditions here to check for role type
    let mut row;
    if (int_user_role > 0) {
        row = sqlx::query(format!("SELECT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE r.id = {};", int_user_role).as_str())
        .fetch_all(&mut *conn)
        .await;
    }
    /*else if (found_user_role=="2"){
        row = sqlx::query(format!("SELECT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE r.id = 2;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }  else if (found_user_role=="3"){
        row = sqlx::query(format!("SELECT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE r.id = 3;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }*/
    else {
        row = sqlx::query(format!("SELECT b.*, pr.name AS provider_name, pr.url AS provider_url, pr.phone AS provider_phone, bs.status AS bill_status FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id JOIN Provider pr ON b.providerid = pr.id JOIN BillStatus bs ON b.billstatusid = bs.id WHERE r.id = 0;").as_str())
        .fetch_all(&mut *conn)
        .await;
    }
    //sets up vector to store bills in
    let mut vec = vec![];
    //unwrap row
    match row {
        Ok(row) => {
            for item in row {
                //could we just instantiate object here? probably
                let bill_item = Bill {
                    id: item.get("id"),
                    amount: item.get("amount"),
                    date: item.get("date"),
                    img_path: item.get("img_path"),
                    duedate: item.get("duedate"),
                    providerid: item.get("providerid"),
                    billstatusid: item.get("billstatusid"),
                    provider_name: item.get("provider_name"),
                    bill_status: item.get("bill_status"),
                };
                vec.push(bill_item);
            }
        }
        Err(e) => {
            //need to get error handling here
            panic!("error finding row")
        }
    }
    let full_string = json_builder(vec.clone());
    HttpResponse::Ok().body(full_string)
}
//endpoint to get users
#[post("/u")]
async fn u(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    //pethaps it might be more effected to decrypt both? who knows
    //this key needs moved to an .env file
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).expect("no cookie");
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    //get db connection
    let mut conn = data.db.acquire().await.unwrap();
    //get all bills
    //should probably add some conditions here to check for role type
    let mut row;
    if (found_user_role == "1") {
        row = sqlx::query(format!("SELECT u.*, r.id AS role_id, rt.role AS role_name FROM Users u JOIN need n ON u.id = n.userid JOIN Roles r ON n.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }
    /* else if (found_user_role=="2"){
        row = sqlx::query(format!("SELECT u.*, r.id AS role_id, rt.role AS role_name FROM Users u JOIN need n ON u.id = n.userid JOIN Roles r ON n.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }*/
    else {
        row = sqlx::query(format!("SELECT ID FROM USERS").as_str())
            .fetch_all(&mut *conn)
            .await;
    }
    //sets up vector to store bills in
    let mut vec = vec![];
    //unwrap row
    match row {
        Ok(row) => {
            for item in row {
                //could we just instantiate object here? probably
                let bill_item = User {
                    id: item.get("id"),
                    username: item.get("username"),
                    role: item.get("role_id"),
                };
                vec.push(bill_item);
            }
        }
        Err(e) => {
            //need to get error handling here
            panic!("error finding row")
        }
    }
    let full_string = json_builder(vec.clone());
    HttpResponse::Ok().body(full_string)
}
//end point to get status types
#[post("/s")]
async fn s(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    //get db connection
    let mut conn = data.db.acquire().await.unwrap();
    //get all bills
    //should probably add some conditions here to check for role typei
    let mut row;
    if (found_user_role == "1") {
        row = sqlx::query(format!("SELECT * FROM BillStatus;").as_str())
            .fetch_all(&mut *conn)
            .await;
    } else if (found_user_role == "2") {
        row = sqlx::query(format!("SELECT * FROM BillStatus;").as_str())
            .fetch_all(&mut *conn)
            .await;
    } else {
        row = sqlx::query(format!("SELECT ID FROM BillStatus;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }
    //sets up vector to store bills in
    let mut vec = vec![];
    //unwrap row
    match row {
        Ok(row) => {
            for item in row {
                //could we just instantiate object here? probably
                let bill_item = BillStatus {
                    id: item.get("id"),
                    status: item.get("status"),
                };
                vec.push(bill_item);
            }
        }
        Err(e) => {
            //need to get error handling here
            panic!("error finding row")
        }
    }
    let full_string = json_builder(vec.clone());
    HttpResponse::Ok().body(full_string)
}
//end point to get providers
#[post("/p")]
async fn p(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    //get db connection
    let mut conn = data.db.acquire().await.unwrap();
    //get all bills
    //should probably add some conditions here to check for role typei
    let mut row;
    if (found_user_role == "1") {
        row = sqlx::query(format!("SELECT * FROM PROVIDER;").as_str())
            .fetch_all(&mut *conn)
            .await;
    } else if (found_user_role == "2") {
        row = sqlx::query(format!("SELECT * FROM PROVIDER;").as_str())
            .fetch_all(&mut *conn)
            .await;
    } else {
        row = sqlx::query(format!("SELECT ID FROM PROVIDER;").as_str())
            .fetch_all(&mut *conn)
            .await;
    }
    //sets up vector to store bills in
    let mut vec = vec![];
    //unwrap row
    match row {
        Ok(row) => {
            for item in row {
                //could we just instantiate object here? probably
                let bill_item = Provider {
                    id: item.get("id"),
                    url: item.get("url"),
                    phone: item.get("phone"),
                    name: item.get("name"),
                };
                vec.push(bill_item);
            }
        }
        Err(e) => {
            //need to get error handling here
            panic!("error finding row")
        }
    }
    let full_string = json_builder(vec.clone());
    HttpResponse::Ok().body(full_string)
}

#[post("/add_user")]
async fn add_user(data: web::Data<AppState>, req: HttpRequest, req_body: String) -> impl Responder {
    let mut conn = data.db.acquire().await.unwrap();
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    //pethaps it might be more effected to decrypt both? who knows
    //this key needs moved to an .env file
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    if (found_user_role != "1") {
        return HttpResponse::Ok().body("Invalid Permissions");
    }
    let nu_user: NewUser = serde_json::from_str(&req_body.as_str()).unwrap();
    let format_username = input_sanitizer(nu_user.username);
    let format_password = input_sanitizer(nu_user.password);
    //convert roleid to int
    let int_role: i32 = nu_user.role.parse().unwrap();
    let user_id: i32 = sqlx::query_scalar(
        format!(
            "INSERT INTO USERS (username, password) VALUES ('{}', '{}') RETURNING id",
            format_username, format_password
        )
        .as_str(),
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    //adds user to the need table so they can be assigned requested role
    sqlx::query(format!("INSERT INTO need (userid, roleid) VALUES ((SELECT id FROM Users WHERE username = '{}' AND password='{}'),('{}'));",format_username, format_password, int_role).as_str())
     .execute(&mut *conn)
        .await;
    HttpResponse::Ok().body(req_body)
}
#[post("/add_bill")]
async fn add_bill(data: web::Data<AppState>, req: HttpRequest, req_body: String) -> impl Responder {
    let mut conn = data.db.acquire().await.unwrap();
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    //pethaps it might be more effected to decrypt both? who knows
    //this key needs moved to an .env file
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    if (found_user_role != "1" && found_user_role != "2") {
        return HttpResponse::Ok().body("Invalid Permissions");
    }
    //convert response to modified bill object
    let nu_bill: StringBill = serde_json::from_str(&req_body.as_str()).unwrap();
    //image conversion to avoid user shenanigans
    let format_img_path = input_sanitizer(nu_bill.img_path);
    //convert date to str
    let format_date: &str = nu_bill.date.as_str();
    //splt date by - to be remodified so the parse_from_str agrees with my american date format
    let mut split = format_date.split('-');
    //set up vector to store splits because i don't understand splits
    let mut vector = vec![];
    //look through splits to store in vector
    for item in split {
        vector.push(item);
    }
    let mut formatted_date: String = String::from("0000-00-00T00:00:00+00:00");
    if vector.len() != 3 {
        formatted_date = String::from("0000-00-00T00:00:00+00:00");
    }
    if vector.len() == 3 {
        //vector[2] is the year, vector[0] is th emonth, vector [1] is the day
        formatted_date = format!("{}-{}-{}T00:00:00+00:00", vector[2], vector[0], vector[1]);
    }
    //finish converting date to avoid user shenanigans
    let date_date: NaiveDateTime =
        NaiveDateTime::parse_from_str(&formatted_date, "%Y-%m-%dT%H:%M:%S%z")
            .expect("Failed to parse date string.");
    //same deal for due date
    //convert date to str
    let format_due_date: &str = nu_bill.duedate.as_str();
    //splt date by - to be remodified so the parse_from_str agrees with my american date format
    let mut split = format_due_date.split('-');
    //set up vector to store splits because i don't understand splits
    let mut vector = vec![];
    //look through splits to store in vector
    for item in split {
        vector.push(item);
    }
    //vector[2] is the year, vector[0] is th emonth, vector [1] is the day
    let formatted_date: String =
        format!("{}-{}-{}T00:00:00+00:00", vector[2], vector[0], vector[1]);
    //finish converting date to avoid user shenanigans
    let date_duedate: NaiveDateTime =
        NaiveDateTime::parse_from_str(&formatted_date, "%Y-%m-%dT%H:%M:%S%z")
            .expect("Failed to parse due date string.");
    //convert amount to float to avoid user shenanigans
    let float_amount: f64 = nu_bill.amount.parse().unwrap();
    //convert providerid to int
    let int_providerid: i32 = nu_bill.providerid.parse().unwrap();
    //convert providerid to int
    let int_billstatusid: i32 = nu_bill.billstatusid.parse().unwrap();
    let bill_id:i32 = sqlx::query_scalar(format!("INSERT INTO BILLS (amount, date, img_path, duedate, providerid, billstatusid) VALUES ('{}', '{}', '{}','{}','{}','{}') RETURNING id", float_amount, date_date,format_img_path, date_duedate,int_providerid, int_billstatusid).as_str())
        .fetch_one(&mut *conn)
        .await.unwrap();
    //takes bill_id and adds all admin users permission to view the added bill, this needs a lot of
    //work
    //need to change from rt.role
    sqlx::query(format!("INSERT INTO Permits (roleid, billid) VALUES ((SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'admin'), (SELECT id FROM Bills WHERE id = '{}'));", bill_id).as_str())
        .execute(&mut *conn)
        .await;
    //adds perms for 'mod' group
    sqlx::query(format!("INSERT INTO Permits (roleid, billid) VALUES ((SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'mod'), (SELECT id FROM Bills WHERE id = '{}'));", bill_id).as_str())
        .execute(&mut *conn)
        .await;
    //add perms for 'user' group
    sqlx::query(format!("INSERT INTO Permits (roleid, billid) VALUES ((SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'user'), (SELECT id FROM Bills WHERE id = '{}'));", bill_id).as_str())
        .execute(&mut *conn)
        .await;
    HttpResponse::Ok().body(req_body)
}
#[post("/edit_bill/{id}")]
async fn edit_bill(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_body: String,
    path: web::Path<(u32,)>,
) -> impl Responder {
    let user_slug: u32 = path.into_inner().0;
    let mut conn = data.db.acquire().await.unwrap();
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    if (found_user_role != "1" && found_user_role != "2") {
        return HttpResponse::Ok().body("Invalid Permissions");
    }
    //convert response to modified bill object
    let nu_bill: StringBill = serde_json::from_str(&req_body.as_str()).unwrap();
    //image conversion to avoid user shenanigans
    let format_img_path = input_sanitizer(nu_bill.img_path);
    //convert date to str
    let format_date: &str = nu_bill.date.as_str();
    //splt date by - to be remodified so the parse_from_str agrees with my american date format
    let mut split = format_date.split('-');
    //set up vector to store splits because i don't understand splits
    let mut vector = vec![];
    //look through splits to store in vector
    for item in split {
        vector.push(item);
    }
    let mut formatted_date: String = String::from("0000-00-00T00:00:00+00:00");
    if vector.len() != 3 {
        formatted_date = String::from("0000-00-00T00:00:00+00:00");
    }
    if vector.len() == 3 {
        //vector[2] is the year, vector[0] is th emonth, vector [1] is the day
        formatted_date = format!("{}-{}-{}T00:00:00+00:00", vector[2], vector[0], vector[1]);
    }
    //finish converting date to avoid user shenanigans
    let date_date: NaiveDateTime =
        NaiveDateTime::parse_from_str(&formatted_date, "%Y-%m-%dT%H:%M:%S%z")
            .expect("Failed to parse date string.");
    //same deal for due date
    //convert date to str
    let format_due_date: &str = nu_bill.duedate.as_str();
    //splt date by - to be remodified so the parse_from_str agrees with my american date format
    let mut split = format_due_date.split('-');
    //set up vector to store splits because i don't understand splits
    let mut vector = vec![];
    //look through splits to store in vector
    for item in split {
        vector.push(item);
    }
    //vector[2] is the year, vector[0] is th emonth, vector [1] is the day
    let formatted_date: String =
        format!("{}-{}-{}T00:00:00+00:00", vector[2], vector[0], vector[1]);
    //finish converting date to avoid user shenanigans
    let date_duedate: NaiveDateTime =
        NaiveDateTime::parse_from_str(&formatted_date, "%Y-%m-%dT%H:%M:%S%z")
            .expect("Failed to parse due date string.");
    //convert amount to float to avoid user shenanigans
    let float_amount: f64 = nu_bill.amount.parse().unwrap();
    //convert providerid to int
    let int_providerid: i32 = nu_bill.providerid.parse().unwrap();
    //convert providerid to int
    let int_billstatusid: i32 = nu_bill.billstatusid.parse().unwrap();
    let bill_id:i32 = sqlx::query_scalar(format!("INSERT INTO BILLS (amount, date, img_path, duedate, providerid, billstatusid) VALUES ('{}', '{}', '{}','{}','{}','{}') RETURNING id", float_amount, date_date,format_img_path, date_duedate,int_providerid, int_billstatusid).as_str())
        .fetch_one(&mut *conn)
        .await.unwrap();
    //takes bill_id and adds all admin users permission to view the added bill, this needs a lot of
    //work
    //need to change from rt.role
    //     sqlx::query(format!("INSERT INTO Permits (roleid, billid) VALUES ((SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'admin'), (SELECT id FROM Bills WHERE id = '{}'));", bill_id).as_str())
    //         .execute(&mut *conn)
    //         .await;
    // //adds perms for 'mod' group
    //     sqlx::query(format!("INSERT INTO Permits (roleid, billid) VALUES ((SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'mod'), (SELECT id FROM Bills WHERE id = '{}'));", bill_id).as_str())
    //         .execute(&mut *conn)
    //         .await;
    //
    //     //add perms for 'user' group
    //     sqlx::query(format!("INSERT INTO Permits (roleid, billid) VALUES ((SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'user'), (SELECT id FROM Bills WHERE id = '{}'));", bill_id).as_str())
    //         .execute(&mut *conn)
    //         .await;
    HttpResponse::Ok().body(req_body)
}

#[post("/add_provider")]
async fn add_provider(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_body: String,
) -> impl Responder {
    let mut conn = data.db.acquire().await.unwrap();
    //set an owned string so it can be converted to str "c"
    let mut owned_string = String::new();
    //gets cookie from client
    let cookie = req.cookie("bones").map(|c| c.value().to_string());
    let cookie_value = cookie_unwrapper(cookie);
    let c: &str = cookie_value.as_str();
    //take key value and encrypt it, so it can be compared
    //pethaps it might be more effected to decrypt both? who knows
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
    //verified the cookie against the key
    let token_usr: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(c, &key).unwrap();
    //deconstruct header...do we need this? maybe
    let header = token_usr.header();
    //deconstruct claims,
    let claims = token_usr.claims();
    //this should probably verify role against the db?
    let found_user_role = claims["role"].clone();
    let found_user_name = claims["sub"].clone();
    if (found_user_role != "1" && found_user_role != "2") {
        return HttpResponse::Ok().body("Invalid Permissions");
    }
    //convert response to modified bill object
    let nu_provider: NewProvider = serde_json::from_str(&req_body.as_str()).unwrap();
    //image conversion to avoid user shenanigans
    let format_url = input_sanitizer(nu_provider.url);
    //image conversion to avoid user shenanigans
    let format_name = input_sanitizer(nu_provider.name);
    let format_phone = input_sanitizer(nu_provider.phone);
    let provider_id: i32 = sqlx::query_scalar(
        format!(
            "INSERT INTO PROVIDER (url, phone, name) VALUES ('{}', '{}', '{}') RETURNING id",
            format_url, format_phone, format_name
        )
        .as_str(),
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    HttpResponse::Ok().body(req_body)
}
#[post("logout")]
async fn logout(data: web::Data<AppState>, req_body: String) -> impl Responder {
    let mut cookie = Cookie::build("bones", "value-does-not-matter")
        .domain("0.0.0.0")
        .path("/")
        .finish();
    cookie.make_removal();
    let res = HttpResponse::Ok().cookie(cookie).finish();
    res
}
#[post("/login")]
async fn login(data: web::Data<AppState>, req_body: String) -> impl Responder {
println!("Received request: {:?}", req_body);
    let user: User_no_auth = serde_json::from_str(req_body.as_str()).expect("not good");
    // add logic here to check if user and password are in db somehow...securely
    // then give them some kind of token or cookie to actually access bills
    //gets db connection
    //
    let mut conn = data.db.acquire().await.unwrap();
    //old don't think i need anymore
    //
   println!("Database connection acquired"); 
    let query_parameter = user.username.clone();
    let parsed_query_parameter: String = input_sanitizer(user.username);
    //users password, this probably needs stored in the db as encrypted data and not plain text
    //also this removes spaces from the user input
    
    let parsed_query_parameter_password: String = input_sanitizer(user.password);
    let mut row = sqlx::query(
        format!(
            "SELECT u.id, u.username, r.id AS role \
         FROM Users u \
         JOIN need n ON u.id = n.userid \
         JOIN Roles r ON n.roleid = r.id \
         WHERE u.username = '{}' AND u.password = '{}';",
            parsed_query_parameter.replace("'", "''"), // Simple escaping by duplicating single quotes
            parsed_query_parameter_password.replace("'", "''") // Simple escaping by duplicating single quotes
        )
        .as_str(),
    )
    .fetch_one(&mut *conn)
    .await;

    match row {
        Ok(row) => {
 println!("Query successful, row fetched");
            let obj = User {
                id: row.get("id"),
                username: row.get("username"),
                role: row.get("role"),
            };

            let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_KEY.as_bytes()).unwrap();
            let header = Header {
                algorithm: AlgorithmType::Hs384,
                ..Default::default()
            };
            let mut claims = BTreeMap::new();
            claims.insert("sub", obj.username.clone());
            claims.insert("role", obj.role.clone().to_string());
            let token_cookie = Token::new(header, claims).sign_with_key(&key).unwrap();
            let mut cookie = Cookie::build("bones", token_cookie.as_str().to_owned())
                .domain("localhost")
                .path("/")
                .secure(true)
                .http_only(true)
                .finish();
            //sets cookie duration to 60 minutes
            cookie.set_max_age(Duration::minutes(60));
            println!("{:?}",cookie);
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "http://0.0.0.0:4173")
                .header("Access-Control-Allow-Credentials", "true")
                .cookie(cookie)
                .body(format!(
                    "{{\"role\":\" {}\",\
                                    \"username\":\"{}\"}}",
                    obj.role, obj.username
                ))
        }
        Err(e) => {
println!("Error executing query: {:?}", e);
            let obj = Error {
                error: e.to_string(),
            };
            let turn_to_json = serde_json::to_string(&obj);
            match turn_to_json {
                Ok(turn_to_json) => HttpResponse::Ok().body(turn_to_json),
                Err(e) => {
                    HttpResponse::Ok().body(format!("{{\"error\":\"error parsing db {}\"}}", e))
                }
            }
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        //need to put this into a .env file
        .connect(POSTGRES_CONN_STRING)
        .await
        .expect("error connecting to postgres");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4173")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b"localhost:4173"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(login)
            .service(bill_edit)
            .service(b)
            .service(add_bill)
            .service(u)
            .service(p)
            .service(s)
            .service(add_user)
            .service(add_provider)
            /*.service(token)*/
            .service(logout)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
