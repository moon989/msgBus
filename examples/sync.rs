use async_std::task;
use futures_timer::Delay;
use msg_bus::*;
use std::sync::Arc;
use std::time::Duration;
use std::error::Error;

use async_trait::async_trait;

#[derive(Clone)]
struct MsgRunner{
    name: String,
}
#[derive(Clone,Debug)]
struct MD {
    name:String,
    id:usize,
    city:String,
}


#[async_trait]
impl MessageHandle<MD> for MsgRunner {
   async fn handle_msg(&self, msg: MessageData<MD>)-> std::result::Result<(),Box<dyn Error>>{
        match msg {
            MessageData::Data(s) => {
                println!("--- i am {:?}-{:?}", self.name, s);
            }
            MessageData::None => {
                println!("--- i am handle massage {}", "None");
            }
        }
        Ok(())
    }
}


#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

        let bus = MsgBus::<MD>::new();
        let arc_bus = Arc::new(bus);
        let ru= MsgRunner{
            name:"lisi".to_string(),
        };
        let bus1 = arc_bus.clone();

        task::spawn(process_reg(bus1, ru));

        let bus2 = arc_bus.clone();
        task::spawn(process_post(bus2));

        let bus3 = arc_bus.clone();
        task::spawn(process_post(bus3));

        println!("spawn over!");


    println!("thread over!");
    std::thread::sleep(Duration::from_secs(99999999999999999));
    println!("sleep over!");
    Ok(())
}

async fn process_reg(arc_bus: Arc<MsgBus<MD>>, hd: MsgRunner) {
    println!("begin proces reg..");
    let mut i: usize = 0;
    loop {
        Delay::new(Duration::from_secs(2)).await;
        i += 1;
        println!("reg message id is..{}",i);
        arc_bus.regist_msg(i, Box::new(hd.clone())).await;
    }
}

async fn process_post(arc_bus: Arc<MsgBus<MD>>) {
    println!("begin proces post..");
    let mut i: usize = 0;
    loop {
        Delay::new(Duration::from_secs(3)).await;
        i=i+1;
        // if msg_type == 0 {
        //     let msg = Message::<String> {
        //         id: i,
        //         data: MessageData::Data(format!("message is {}", i)),
        //     };
        //     arc_bus.post_msg(msg).await.unwrap();
        // } else
         {
            let md = MD{
                name:"Name".to_string(),
                id: i,
                city: format!("city-{}",i),
            };
            let msg = Message::<MD> {
                id: i,
                data: MessageData::Data(md),
            };
            arc_bus.post_msg(msg).await.unwrap();
        }

        println!("----- post message id is..{}",i);
       
    }
}
