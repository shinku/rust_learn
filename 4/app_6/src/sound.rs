pub fn guitar(){
    println!("{}","guitar");
    instrument::woodwind::clarinet();
}
pub mod instrument{
    pub mod  woodwind{
        pub fn clarinet(){
            println!("{}","clarinet");
            super::show_p();
        }   
    }
    pub fn show_p(){
        println!("{}","showP");
        super::show_n();
    }
}
pub fn show_n(){
    println!("shown");
}
mod voice{

}