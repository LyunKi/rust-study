use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[test]
fn test_thread_1() {
    let a = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    a.join().unwrap();
}

#[test]
fn test_thread_2() {
    let v = vec![1, 2, 3, 4];
    thread::spawn(move || {
        println!("here is a vector:{:?}", v);
    });
    //it's not allowed
//    drop(v);
}


#[test]
fn test_channel_1() {
    let (tx, rx) = mpsc::channel();
    let value = String::from("hello");
    thread::spawn(move || {
        tx.send(value).unwrap();
        //value的所有权被转移了，所以下列代码不能编译
//        assert_eq!(value, "hello");
    });
    let recv_msg = rx.recv().unwrap();
    assert_eq!(recv_msg, "hello");
}

#[test]
fn test_channel_2() {
    let (tx, rx) = mpsc::channel();
    let messages = vec![1, 2, 3, 4];
    let send_msgs = messages.clone();
    //发送者可以有多个，通过clone来创建
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        for msg in send_msgs {
            tx1.send(msg).unwrap();
        };
    });
    let mut i = 0;
    for recv in rx {
        assert_eq!(messages[i], recv);
        i += 1;
    }
}

#[test]
fn test_channel_3() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(10, *counter.lock().unwrap());

}
