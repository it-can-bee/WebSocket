# Rust WebSocket
## 项目简介
### 1、用Map存储不同的用户
```
* Key: id(usize)
* Value: 发送给前端不同用户的渠道
```

### 2、通过Filter system，wrap实现了
```
* websocket以及on_upgrade方法
* websockets以及split方法
```

### 3、通过tokio的channel获取所有前端(用户)发送的信息(WebSocket)转发给Map中的Value

## 运行
```
cargo run
```

## 运行结果
我启动了三个浏览器窗口进行信息发送和转发，具体如下
```
每个窗口对应一个uid，发送的msg Text为对应内容，用uid是为了防止并发情况下，对于消息对象信息处理的紊乱
```
```
 INFO  warp::server > Server::run; addr=127.0.0.1:7070
 INFO  warp::server > listening on http://127.0.0.1:7070
 DEBUG hyper::proto::h1::io > parsed 15 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG warp::filters::fs      > dir: appending index.html to directory path
 DEBUG hyper::proto::h1::io   > flushed 171 bytes
 DEBUG hyper::proto::h1::io   > flushed 1142 bytes
 DEBUG hyper::proto::h1::io   > parsed 14 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 177 bytes
 DEBUG hyper::proto::h1::io   > flushed 1638 bytes
 DEBUG hyper::proto::h1::io   > parsed 12 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 166 bytes
 INFO  ws_code                > Connected UserId by 1
 DEBUG hyper::proto::h1::io   > parsed 14 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG warp::filters::fs      > file not found: "static/favicon.ico"
 DEBUG warp::filter::service  > rejected: Rejection(NotFound)
 DEBUG hyper::proto::h1::io   > flushed 82 bytes
 INFO  ws_code                > uid is 1 and message is Text("11111")
 DEBUG hyper::proto::h1::io   > parsed 12 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 166 bytes
 INFO  ws_code                > Connected UserId by 2
 INFO  ws_code                > uid is 1 and message is Text("222")
 INFO  ws_code                > uid is 2 and message is Text("222")
 DEBUG hyper::proto::h1::io   > parsed 12 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 166 bytes
 INFO  ws_code                > Connected UserId by 3
 INFO  ws_code                > uid is 1 and message is Text("333")
 INFO  ws_code                > uid is 3 and message is Text("333")
 INFO  ws_code                > uid is 2 and message is Text("333")
 DEBUG hyper::proto::h1::conn > read eof

```
