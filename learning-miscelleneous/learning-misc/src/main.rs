struct Point<T,U>{
    x:T,
    y:U

}

impl <T,U> Point<T,U>{
    fn readout(self)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,   
    {
        println!("The point is: {}, {}",self.x,self.y);

    }
    fn add_two(self) ->U
    where
            T: std::ops::Add<U, Output = U>,
    {
        self.x+self.y
    }
}

fn main(){

    let my_int_Point = Point{x:1,y:2};
    let my_float_Point = Point{x:2.0,y:3.1};
    let my_mixed_Point = Point{x:1,y:2.4};

    my_mixed_Point.readout();

    println!("{:?}",my_mixed_Point.add_two());

}