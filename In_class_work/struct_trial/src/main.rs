#[derive(Debug)]
struct Car{
    body: String,
    year: u16,
    color: String,
}

impl Car {
    fn new(b:String,y:u16,c:String) -> Car {
        Car{
            body: b,
            year: y,
            color: c,
        }
    }
    fn show_info(&self){
        println!("{}\n{}\n{}", self.body,self.year,self.color);
    }

    fn change_color(&mut self, new_color:String){
        self.color = new_color;
    }
}

fn car_info(car:&Car){
    println!("{}\n{}\n{}\n",car.body,car.year,car.color);
}

fn main() {
    /*let my_car = Car{
        body: "Sedan".to_string(),
        year: 2020,
        color: "purple".to_string(),
    };
    car_info(&my_car);
    car_info(&my_car);
    // */

    //* 
    let mut my_car = Car::new("Sedan".to_string(),2020,"Purple".to_string());
    println!("{:?}",my_car);
    my_car.change_color("BLACK".to_string());
    my_car.show_info();
    //*/

}
