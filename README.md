
# msg_bus

## V0.0.2

## 介绍

msg_bus 异步事件消息处理总线，类似window 消息处理机制，通过注册消息，发送消息，消息响应。可以应用在多线程之间消息的传递。支持自定义消息数据类型，使用简单。使用示例如下。详细的例子参见 examples 目录。

```rust

#[async_trait]
impl MessageHandle<String> for MsgRunner{

    async fn handle_msg(&self, msg: MessageData<String>) ->std::result::Result<(),Box<dyn std::error::Error>>{
        match msg {
            MessageData::Data(s) => {
                println!("--- i am handle massage {}",s);
            },
            MessageData::None =>{
                println!("--- i am handle massage {}","None");
            }
        }
        Ok(())
    }

}
#[async_std::test]
async fn test_register() {
    let bus = MsgBus::<String>::new();

    let ru = MsgRunner{};
    bus.regist_msg(1, Box::new(ru)).await;
    let msg1 =Message{id:1,data:MessageData::Data("aaaaa".to_string())};

    bus.post_msg(msg1).await.unwrap();

    let msg2 =Message{id:1,data:MessageData::Data("bbb".to_string())};
    bus.post_msg(msg2).await.unwrap();
    // 等候下异步执行线程，输出结果；
    sleep(std::time::Duration::from_secs(5));
    println!("Done!");


}
```

欢迎大家共同完善，联系作者，请发邮件 2175318066@qq.com
