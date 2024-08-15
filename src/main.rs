use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use futures::StreamExt;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter; 
use log::info;

//key id 原子计数+1
static NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

//value Message Sender 多线程 读写锁  哈希map（id：sender） sender的channel非堵塞
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

/* Web 服务器会在本地的 7070 端口提供 static 目录中的静态文件 
* 通过 tokio 的 channel 把不同用户端之间的信息传递(WebSocket)存储在Map的V中
*/
#[tokio::main]
async fn main() {

    // 设置环境变量 "RUST_APP_LOG" 的值为 "debug"
    env::set_var("RUST_APP_LOG", "debug");
    // 初始化日志记录器，使用 "RUST_APP_LOG" 环境变量指定的日志级别
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let users = Users::default();
    //加标注指定websocket请求而不是http请求 然后希望传入users
    let chat = warp::path("ws")
        .and(warp::ws())
        .and(with_user(users))
        .map(|ws:warp::ws::Ws, users| ws.on_upgrade(move |socket| conn(socket, users))); /*获取某些东西*/
    
    // 创建一个 Warp 过滤器，指向 "static" 目录
    let files = warp::fs::dir("static");
    //建立添加路由
    let router = chat.or(files);
    // 使用 Warp 提供静态文件服务，监听本地地址 127.0.0.1 的 7070 端口
    warp::serve(router).run(([127, 0, 0, 1], 7070)).await;

    println!("Hello world");
}


//异步处理users 操作时进行升级
fn with_user(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    //通过clone将users移进去
    warp::any().map(move || users.clone())
}

//异步连接和发送消息
async fn conn(ws: WebSocket, users: Users) {
    //拿到id去建立连接
    let my_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    info!("Connected UserId by {}", my_id);
    //分离流信息
    let (user_tx, mut user_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    tokio::spawn(rx.forward(user_tx));
    //add users
    users.write().await.insert(my_id, tx);

    //读取和广播发送消息
    while let Some(result) = user_rx.next().await {
        broadcast(result.unwrap(), &users).await;
    }
    disconn(my_id, &users).await;
}

//拿到users这个map的msg 广播消息send
async fn broadcast(msg: Message, users: &Users) {
    if let Ok(_) = msg.to_str() {
        for (&uid, tx) in users.read().await.iter() {
            info!("uid is {} and message is {:?}", uid, msg.clone());
            tx.send(Ok(msg.clone())).expect("Failed to send message!");
        }
    }
}

//断开链接
async fn disconn(my_id: usize, users: &Users) {
    info!("GoodBye {}", my_id);
    users.write().await.remove(&my_id);
}