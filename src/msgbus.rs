use async_std::sync::RwLock;
use async_std::task;
use futures::channel::mpsc;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;


pub enum MessageData<TMSGDATA> {
    Data(TMSGDATA),
    None,
}

pub struct Message<TMSGDATA> {
    pub id: usize,
    pub data: MessageData<TMSGDATA>,
}

#[async_trait]
pub trait MessageHandle<TMSGDATA>: Send + Sync + 'static {
    async fn handle_msg(&self, msg: MessageData<TMSGDATA>)->std::result::Result<(),Box<dyn Error>>;
}

///
/// register 注册消息，把消息 和 处理 handle 放入 message_heap 队列
/// MsgBus.postMessage 发送消息，把消息 发送到 Channel,内部 dispatch 消息，
/// 并根据消息 查找对应的 handle ，调用 handle 处理.
///
#[allow(dead_code)]

pub struct MsgBus<TMSGDATA> {
    // 注册的消息队列，hashmap 存储消息,和 消息的 处理者
    //outer Arc allows us to clone . inner Arc allows us to clone MessageHandle
    registed_message_heap: Arc<RwLock<HashMap<usize, Arc<Box<dyn MessageHandle<TMSGDATA>>>>>>,
    
    // 防止 Receiver 被多线程征用
    receiver: Arc<RwLock<mpsc::Receiver<Message<TMSGDATA>>>>,

    // Sender 本身具备 Clone 属性，不用Arc
    senders: mpsc::Sender<Message<TMSGDATA>>,
}

// 实现消息Bus
impl<TMSGDATA: Send + Clone + Sync + std::fmt::Debug+'static> MsgBus<TMSGDATA> {
    pub fn new() -> MsgBus<TMSGDATA>
    {
        let (tx, rx) = mpsc::channel::<Message<TMSGDATA>>(10);
        let rx = Arc::new(RwLock::new(rx));
        let hm: HashMap<usize, Arc<Box<dyn MessageHandle<TMSGDATA>>>> = HashMap::new();
        let bus = Self {
            registed_message_heap: Arc::new(RwLock::new(hm)),
            receiver: rx,
            senders: tx,
        };
        let bus2 = bus.clone();
        task::spawn(MsgBus::_dispatch(bus2));
        bus
    }

    // 注册消息，覆盖模式没有考虑消息ID 存在的情况下
    // TMSGDATAODO 消息ID 存在，给出错误
    pub async fn regist_msg(&self, msg_id: usize, handle: Box<dyn MessageHandle<TMSGDATA>>) {
        self.registed_message_heap.write().await.insert(msg_id, Arc::new(handle));
    }

    //删除消息 msg_id 消息ID
    pub async fn remove_msg(&self, msg_id: usize) {
        self.registed_message_heap.write().await.remove(&msg_id);
    }

    // 分发处理消息线程
    async fn _dispatch(self) {
        //println!("dispatch mas");
        loop {
            while let Ok(Some(msg)) = self.receiver.write().await.try_next() {
                //println!("In spawn with msg:");
                match self.registed_message_heap.read().await.get(&msg.id) {
                    Some(handle) => {
                        let data = msg.data;
                        //DONE  修改为异步处理 消息响应事件，提高性能；
                        inner_handle(handle,data);
                    }
                    None => {
                        //debug!("no message ");
                    }
                };
            }
        }
    }

    pub async fn post_msg(&self, msg: Message<TMSGDATA>) -> Result<(), Box<dyn Error>> {
        let mut sender = self.senders.clone();
        match sender.try_send(msg) {
            Ok(()) => return Ok(()),
            _ => return Err("err".into()),
        };
    }
}


impl<M: Send + Clone + Sync + std::fmt::Debug> Clone for MsgBus<M> {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.clone(),
            senders: self.senders.clone(),
            registed_message_heap: self.registed_message_heap.clone(),
        }
    }
}


fn inner_handle<State: Clone + Send + Sync + 'static>(hd: &Arc<Box<dyn MessageHandle<State>>>,msg:MessageData<State>) {
    let handle = hd.clone();
    task::spawn(async move {
        let r = handle.handle_msg(msg);
        if let Err(error) = r.await {
            log::error!("do handle message Err error {} ", { error.to_string() });
        }
    });
}