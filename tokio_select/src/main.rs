use tokio::sync::mpsc;

use get_time::get_time_string;

#[tokio::main]
async fn main() {
    let thread1_parm = 1;

    println!("{}:プログラム開始", get_time_string());

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    println!("{}:thread1_parm:{}", get_time_string(), thread1_parm);

    let (tx_main, rx_main) = mpsc::channel(16);
    let (tx_sub_1, mut rx_sub_1) = mpsc::channel(16);

    let sub_1_sender = tx_main.clone();

    tokio::spawn(async move{
        thread_1(rx_sub_1,sub_1_sender).await
    });

    while let Some(cmd) = rx.recv().await {
        match cmd {
            thread_1::Command::Get { key, resp } => {
                let res = client.get(&key).await;
                // エラーは無視する
                let _ = resp.send(res);
            }
            thread_2::Command::Set { key, val, resp } => {
                let res = client.set(&key, val.into()).await;
                // エラーは無視する
                let _ = resp.send(res);
            }
        }
    }
}

fn 