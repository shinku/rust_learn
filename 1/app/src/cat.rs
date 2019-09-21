
pub mod _cat{
    #[derive(Debug)]
    pub enum CATTYPE {
        //缅因猫
        MIANYIN,
        //波斯猫
        MOSI,
    }

    #[derive(Debug)]
    pub enum CATSEX {
        GIRL,
        BOY,
    }
    #[derive(Debug)]
    pub struct NEWCAT {
        cattype: CATTYPE,
        sex:CATSEX
    }
    impl NEWCAT {
        // add code here
        fn show(&self){
            println!("{:?}",self.cattype)
        }
    }
    pub fn fund_cat()-> NEWCAT {
       let _cat = NEWCAT {
           cattype: CATTYPE::MIANYIN,
           sex:CATSEX::BOY
       };
       _cat.show();
       return _cat;

    }
}