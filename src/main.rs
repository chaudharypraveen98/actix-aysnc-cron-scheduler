use std::{collections::HashMap, time::Duration, str::FromStr};
use chrono::{Local, FixedOffset};
use cron::Schedule;
use reqwest;
use actix_rt::{self, time};

use actix_web::{web, App, HttpServer};

async fn get_ips() -> HashMap<String, String> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();
    resp
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // actix_rt::spawn(async {
    //     let mut interval = time::interval(Duration::from_secs(20));
    //     loop {
    //         interval.tick().await;
    //         let result = get_ips().await;
    //         println!("20 sec {:?}",result);
    //     }
    // });

    actix_rt::spawn(async move {
        let expression = "1/50   *   *     *       *  *  *";
        let schedule = Schedule::from_str(expression).unwrap();
        let offset = Some(FixedOffset::east(0)).unwrap();

        loop {
            let mut upcoming = schedule.upcoming(offset).take(1);
            actix_rt::time::sleep(Duration::from_millis(500)).await;
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {
                    
                    let result = get_ips().await;
                    println!("{:?}",result);
                }
            }
        }
    });
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}