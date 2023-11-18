trait MyTrait{
    fn method(&self);
}

impl MyTrait for i32{
    fn method(&self) {
        println!("调用i32的方法，x={}",*self)
    }
}

impl MyTrait for String{
    fn method(&self) {
        println!("调用String的方法，x={}",*self)
    }
}

impl MyTrait for f64 {
    fn method(&self) {
        println!("调用 f64 的方法，x = {}", *self);
    }
}

fn call_methods(x: &dyn MyTrait){
    x.method();
}

pub fn hw42(){
    let vec:Vec<Box<dyn MyTrait>>=vec![
        Box::new(10i32),
        Box::new("hello".to_string()),
        Box::new(3.1415926f64),
    ];

    for x in vec{
        call_methods(&*x);
    }
    println!("-----------------------------------------------------");
}