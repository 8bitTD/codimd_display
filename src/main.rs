use actix_web::{App, HttpResponse, HttpServer, get, web};
use serde::{Serialize, Deserialize};
use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;

const ADDRESS: &str = "192.168.11.5:8000"; //CodimMDの全体表示URL(IPアドレスとポート番号は適当です)
const OWNER: &str = "hackmd"; //CodiMDのDBのOWNER名
const DBNAME: &str = "hackmd"; //CodiMDのDB名
const DBPASSWORD: &str = "hackmdpass"; //CodiMDのDBのパスワード
const CODIMDADDRESS: &str = "http://hack:3000/"; //CodiMDのURL

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    url: String,
    shortid: String,
    contents: Vec<String>,
    title: String,
    c_time: String,
    u_time: String,
}

async fn get_info(db: &deadpool_postgres::Pool) -> Vec<Info> {
    let ct = db.get().await.unwrap();
    let items = ct.query(r#"select "id", content, title, shortid, "createdAt", "updatedAt" from "Notes" order by "updatedAt" DESC"#, &[]).await.unwrap();
    let mut data:Vec<Info> = vec![];
    for item in items{
        let t_url: uuid::Uuid = item.get("id");
        let t_url = t_url.to_string().replace("-","");
        let mut p_url = t_url.from_hex().unwrap().as_slice().to_base64(base64::STANDARD);
        p_url.pop().unwrap();
        p_url.pop().unwrap();
        p_url = p_url.replace("/","_");
        let url = format!("{}{}", CODIMDADDRESS, &p_url);
        let shortid = item.get("shortid");
        let content: String = item.get("content");
        let title:String = item.get("title");
        let now: std::time::SystemTime = item.get("createdAt");
        let now: chrono::DateTime<chrono::Utc> = now.into();
        let now = now.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()).naive_local();
        let now = now.format("%Y/%m/%d/%H:%M:%S").to_string();
        let c_time = format!("{}",&now);
        let now: std::time::SystemTime = item.get("updatedAt");
        let now: chrono::DateTime<chrono::Utc> = now.into();
        let now = now.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()).naive_local();
        let now = now.format("%Y/%m/%d/%H:%M:%S").to_string();
        let u_time = format!("{}",&now);
        data.push(Info{url: url, shortid: shortid, contents: vec![content], title: title, c_time: c_time, u_time: u_time});
    }
    data
}

#[get("/codimd_display")]
async fn get_display_html (db: web::Data<deadpool_postgres::Pool>, tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, actix_web::Error>{
    let data = get_info(&db).await;
    let mut ctx = tera::Context::new();
    ctx.insert("data",&data);
    let view = tmpl.render("display.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cfg = tokio_postgres::Config::new();
    cfg.host("localhost");
    cfg.user(OWNER);
    cfg.password(DBPASSWORD);
    cfg.dbname(DBNAME);
    let mgr = deadpool_postgres::Manager::new(cfg, tokio_postgres::NoTls);
    let pool_b = deadpool_postgres::Pool::builder(mgr).max_size(100);
    let pool = pool_b.build().unwrap();
    HttpServer::new( move || {
        let tera = tera::Tera::new(concat!("templates/**/*")).unwrap();
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(tera))
        .service(get_display_html)
    })
    .bind(ADDRESS)?
    .run()
    .await
}