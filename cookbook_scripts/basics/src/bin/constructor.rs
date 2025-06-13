struct NameLength{
    //one can use Cow<'a,str> so as to take &str as well as String
    name:String,
    length:i32,
}

impl NameLength {
    fn new(name : &str) -> Self{
        NameLength {  length: name.len() as i32,name:name.to_string()}
    }

    fn print(&self){
        println!("The name {} has length of {}",self.name,self.length)
    }
    
}

fn main(){

    let name1 = NameLength::new("name");

    name1.print();

}