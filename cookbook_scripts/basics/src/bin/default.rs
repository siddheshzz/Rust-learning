enum CrustType{
    Thin,
    Thick,
}

#[derive(Default)]
struct PizzaConfig{
    wants_cheese: bool,
    number_of_olives: i32,
    special_message: String,
    crust_type: CrustType,
}

impl  Default for CrustType {
    fn default() -> Self {
        CrustType::Thin
    }
    
}

fn main(){
    let intfoo: i32 = Default::default();
    println!("{}", intfoo);

    let pizza: PizzaConfig = Default::default();
    println!("wants cheese {}",pizza.wants_cheese);

    println!("number of olives {}", pizza.number_of_olives);

    println!("special message - {}",pizza.special_message);

    let crust_type = match pizza.crust_type{
        CrustType::Thick => "Thick and extra",
        CrustType::Thin => "Thin and crisp",
    };

    println!("crust type - {}",crust_type);

    // one can aslo configure only certain values
    let olive_pizza = PizzaConfig{
        number_of_olives: 12,
        ..Default::default()
    };

}