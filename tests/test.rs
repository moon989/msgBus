 
use std::thread::sleep;

use msg_bus::*;
use async_trait::async_trait;

struct MsgRunner{}

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
