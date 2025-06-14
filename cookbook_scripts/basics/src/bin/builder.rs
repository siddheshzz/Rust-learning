use std::fmt::Error;

struct Burger{
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl Burger {
    fn print(&self){
        let pretty_patties = if self.patty_count == 1{
            "patty"
        }else{
            "patties"
        };

        let pretty_bool = |val| if val {""} else {"no"};
        let pretty_vegetarian = if self.vegetarian{
            "vegetarian"
        } else{
            ""
        };
        println!(
            "This is a {} burger with  {}, {}, {} cheese, {} bacon and {} salad",
            pretty_vegetarian,
            self.patty_count,
            pretty_patties,
            pretty_bool(self.cheese),
            pretty_bool(self.bacon),
            pretty_bool(self.salad)
        )
    }
    
}

struct BurgerBuilder{
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl BurgerBuilder{
    fn new() -> Self{
        BurgerBuilder { patty_count: 1, vegetarian: false, cheese: false, bacon: false, salad: false }
    }
    fn patty_count(mut self,count:i32) -> Self{
        self.patty_count = count;
        self
    }

    fn cheese(mut self, val :bool) -> Self{
        self.cheese = val;
        self
    }

    fn vegetarian(mut self, val :bool) -> Self{
        self.vegetarian = val;
        self
    }

    fn salad(mut self, val :bool) -> Self{
        self.salad = val;
        self
    }
    fn bacon(mut self, val :bool) -> Self{
        self.bacon = val;
        self
    }

    fn build(&self) -> Result<Burger, String>{
        let burger = Burger{
            patty_count:self.patty_count,
            vegetarian:self.vegetarian,
            cheese:self.cheese,
            bacon:self.bacon,
            salad: self.salad
        };

        if burger.vegetarian && burger.bacon{
            Err("Sorry we do not serve vegetarian bacons yet".to_string())
        }else {
            Ok(burger)
        }

    }
}
fn main(){
    let normal_burgar = BurgerBuilder::new().build();

    let cheese_burger = BurgerBuilder::new()
                                        .cheese(true)
                                        .salad(false)
                                        .build();

    let veg_burger = BurgerBuilder::new()
                                    .vegetarian(true)
                                    .patty_count(2)
                                    .build();

    if let Ok(normal_burger) = normal_burgar{
        normal_burger.print();
    }
    if let Ok(cheese_burger) = cheese_burger{
        cheese_burger.print();
    }
    if let Ok(veg_burger) = veg_burger{
        veg_burger.print();
    }

    // /invalid config

    let invalid_burger = BurgerBuilder::new()
                        .vegetarian(true)
                        .bacon(true)
                        .build();
    
    if let Err(error) = invalid_burger{
        println!("Failed to print burger: {}", error);
    } 
}