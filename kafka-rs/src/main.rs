
extern crate kafka;
extern crate env_logger;

use std::time::Duration;
use std::thread;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use std::str;

fn main() -> Result<(),KafkaError> {
   env_logger::init();
   let broker = "192.168.0.63:9092";
   let topic = "test.topic";

   let data = "hello shiloh".as_bytes();
   produce_message(data, topic, vec![broker.to_owned()])
}

// 生产者
pub fn produce_message<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);
    let mut producer = 
        Producer::from_hosts(brokers)
             // ~ give the brokers one second time to ack the message
             .with_ack_timeout(Duration::from_secs(1))
             // ~ require only one broker to ack the message
             .with_required_acks(RequiredAcks::One)
             // ~ build the producer with the above settings
             .create()
             .unwrap();
    for _ in 0..10000 {
        producer.send(&Record::from_value(topic, data)).unwrap();
        thread::sleep(Duration::from_millis(500))
    }
    Ok(())
}

// 消费者
pub fn consumer_message<'a, 'b> (
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError>  {
    let mut consumer =
    Consumer::from_hosts(brokers)
        .with_topic(topic.to_owned())
        // .with_topic_partitions(topic.to_owned(), &[0, 1])
        // .with_fallback_offset(FetchOffset::Earliest)
        // .with_group("my-group".to_owned())
        // .with_offset_storage(GroupOffsetStorage::Kafka)
        .with_fetch_max_bytes_per_partition(999999999)
        .create()
        .unwrap();
  
        loop {
            let mss = consumer.poll().unwrap();
    
            for ms in mss.iter() {
                for m in ms.messages() {
                    println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, str::from_utf8(m.value).unwrap());
                }
                let _ = consumer.consume_messageset(ms);
            }
            consumer.commit_consumed().unwrap(); // 标记消费
        }
}


mod test {
    use super::{produce_message,consumer_message};
    #[test]
    fn produce_test() {
        env_logger::init();
        let broker = "localhost:9092";
        let topic = "test";
    
        let data = "hello shiloh".as_bytes();
        if let Err(e) = produce_message(data, topic, vec![broker.to_owned()]) {
            println!("{}", e)
        }
    }

    #[test]
    fn consumer_test() {
        env_logger::init();
        let broker = "localhost:9093";
        let topic = "shiloh-opc";

        if let Err(e) = consumer_message(topic, vec![broker.to_owned()]) {
            println!("{}", e)
        }
    }

}