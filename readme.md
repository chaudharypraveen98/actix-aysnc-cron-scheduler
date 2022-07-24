# Scheduling Async Tasks made easy with Actix Cron.
Cron is simply a scheduling daemon that executes the task at a specified interval. We usually use cron for backing up the data, report generation, synchronizing setting in background etc.

**What will we learn : -**
1. Setting Up Actix-web.
2. Adding duration for periodic tasks.
3. Adding cron parser for scheduling flexibility.
4. Performing DB operations using cronjobs.

**Libraries required :-**
* [actix-web](https://actix.rs/) :- A powerful, pragmatic, and extremely fast web framework for Rust
* [reqwest](https://crates.io/crates/reqwest) :- higher level HTTP client library
* [actix-rt](https://crates.io/crates/actix-rt) :- Tokio-based single-threaded async runtime for the Actix ecosystem
* [cron](https://crates.io/crates/cron) :- A cron expression parser and schedule explorer.
* [chrono](https://crates.io/crates/chrono) :- Date and time library for Rust

## 1. Setting Up Actix-web

### Initializing the Rust Project
Start a new project with following `cargo new <file-name>`.

### Implementing basic actix server
Let's clone the sample code from [actix official docs](https://actix.rs/).

### Add dependency in cargo.toml
```
....
[dependencies]
actix-web = "4"
....
```

Let's write code for actix server.

```
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
    
## 2. Adding duration for periodic tasks.
```
use std::{collections::HashMap, time::Duration};
use reqwest;
use actix_rt::time;

// async function to get data from url using reqwest library

async fn get_ips() -> HashMap<String, String> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();
    resp
}

async fn main() -> std::io::Result<()> {

  actix_rt::spawn(async {
      let mut interval = time::interval(Duration::from_secs(20));
      loop {
          interval.tick().await;
          let result = get_ips().await;
          println!("20 sec {:?}",result);
      }
  });
  ....
}
```

### Confused??

No, need to get confused. Let's try to understand step by step code.

**actix_rt ()**
It is a tokio-based single threaded async runtime for actix system. 

Single threaded means only one command get executed at a time.

**actix_rt::spawn** : Spawns a future on the current thread as a new task. If not immediately awaited, the task can be cancelled using JoinHandle::abort (it detaches the current thread when dropped).

**actix_rt::spawn(async move {})** :  move will let you capture closure's environment variable.

Closures are simple anonymous function 
that can be store in variable and doesn't require you to annotate the types.

**tokio time::interval** : Creates new Interval that yields with interval of duration. The first tick completes immediately.

- **interval.tick()** : Completes when the next instant in the interval has been reached.


**Duration::from_secs(20)** : Creates a new Duration with the input whole number in seconds.

## 3. Adding cron parser for scheduling flexibility.

```
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
```

let's try to understand what new happening here

**cron expression** :  do follow the cron pattern. If you want to learn more about it [here](https://crontab.guru/).

```
//                  sec  min   hour   day of month   month   day of week   year
  let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
```

**Schedule::from_str** : IT will parse the value from string.

**FixedOffset** : The fixed offset is used to create date instance. Positive secs means Northern Hemisphere and negative secs means the Western Hemisphere.

**schedule.upcoming(offset).take(1)** : upcoming will take the offset instance and returns an iterator of the DateTime object which matches the schedule.

* **take** : it returns the first item in iterator.

**thread::sleep** : it is necessary to sleep before we check conditions because checking condition at every milli seconds is not ideal as our use case is in seconds or greater than that. If you want minute milli seconds control over cron you can try lower time too.

**Local::now()** : It will return the current datetime depending on local timezone.

**if let Some(datetime) = upcoming.next() {if datetime.timestamp() <= local.timestamp() {} }** : If we find a datetime object in iterator and local datetime(current) is >= the iterator one then it will run the function.

## 4. Performing DB operations using cronjobs.
It's pretty similar just like above two. I will try to give you the glimpse and the example code to understand by your self. 

We will use the clousure(move) over the function to get the db pool and logger.

Please find the code here [actix-question-bank-stackoverflow](https://github.com/chaudharypraveen98/actix-question-bank-stackoverflow/blob/master/src/main.rs#L74:L96).

Special shoutout to [cronjob](https://crates.io/crates/cronjob). It inspires me to write simple cron scheduler.

Feel free to ask queries and make pull request for changes and suggestion in GitHub.

[Github - Source Code](https://github.com/chaudharypraveen98/actix-aysnc-cron-scheduler)


**Happy Hacking**  
Rustaceans!