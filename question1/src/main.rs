mod calender{
    pub mod month{
        pub fn week(){
            println!("There are seven days a week");
        }
    }
}
fn main() {
    println!("Hello, world!");
    calender::month::week();
}
