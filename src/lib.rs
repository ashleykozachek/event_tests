extern crate event;
use std::rc::Rc;

pub struct TestEventStruct<'a> {
    pub s: &'a str,
    pub num: i32,
}

fn test_function<'a,'b>(te: &'a event::Event<TestEventStruct<'b>>){
    match te{
        &event::Event::Args(ref e) => println!("The message is: {} And the number is: {}", e.s, e.num),
        &event::Event::Missing => println!("The event was empty!")
    }
}

#[test]
fn it_works() {
    println!("It imported Events!");
    let test_struct = TestEventStruct { s: "Delicious rustaceans!", num: 10 };
    let test_event = event::Event::Args(test_struct);
    let christmas_present = Box::new(test_function) as Box<for<'a> Fn(&'a event::Event<TestEventStruct>)>; 
    let mut test_publisher = event::EventPublisher::<TestEventStruct>::new();
    
    test_publisher.subscribe_handler(christmas_present);
    
    test_publisher.publish_event(&test_event);
}