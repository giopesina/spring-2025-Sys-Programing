enum Insurance{
    Car(String),
    House(u16),
}
impl Insurance{
    fn show_info(&seld){
        match self{
            Insurance::Car(model) =>println!("My car model is{:?}",model),
            Insurance::House(year) => println!("My house was built in()", year),
        }
    }
}

