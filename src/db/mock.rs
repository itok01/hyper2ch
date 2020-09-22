use super::client::get_db_name;
use crate::models::{Bbs, Message, Thread};
use chrono::prelude::*;
use tokio_postgres::Error;

/// Create mock in database
pub async fn create_mock() -> Result<(), Error> {
    let db_name = get_db_name();
    println!("Creating mock in database {}...", db_name);

    // Create mock BBSes
    let bbs_dog = Bbs::create("犬", "dog", "犬について話す", "動物").await?;
    Bbs::create("猫", "cat", "猫について話す", "動物").await?;
    Bbs::create("鳥", "bird", "鳥について話す", "動物").await?;
    Bbs::create("ロボット", "robot", "ロボットについて話す", "Tech").await?;

    // Create mock threads
    let thread_dog = Thread::create(bbs_dog.id, "犬スレ 7匹目").await?;
    let thread_dog_snack = Thread::create(bbs_dog.id, "犬のおやつスレ 2個目").await?;
    let thread_dog_become = Thread::create(bbs_dog.id, "犬なりきりスレ").await?;

    // Create mock messages
    Message::create(
        thread_dog.id,
        "774匹ワンちゃん",
        "",
        "IAMDOG00",
        "192.168.1.2",
        "example.com",
        "Dog Browser",
        Local::now(),
        "立てました",
    )
    .await?;
    Message::create(
        thread_dog_snack.id,
        "774匹ワンちゃん",
        "",
        "IAMDOG00",
        "192.168.1.2",
        "example.com",
        "Dog Browser",
        Local::now(),
        "立てました",
    )
    .await?;
    Message::create(
        thread_dog_become.id,
        "774匹ワンちゃん",
        "",
        "IAMDOG00",
        "192.168.1.2",
        "example.com",
        "Dog Browser",
        Local::now(),
        "立てました",
    )
    .await?;
    Message::create(
        thread_dog.id,
        "774匹ワンちゃん",
        "",
        "IAMDOG00",
        "192.168.1.2",
        "example.com",
        "Dog Browser",
        Local::now(),
        "ありがとう",
    )
    .await?;

    Ok(())
}
