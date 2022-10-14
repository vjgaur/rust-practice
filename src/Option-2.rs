fn main (){
    
  
    maybe_add_four(Some(i32::from(5)));
    maybe_add_four(None);
    maybe_add_four_v2(Some(i32::from(6)));
    maybe_add_four_v2(None); 
        
    }
    
    
    fn add_four(x: i32) -> i32 {
       println!("{}",x+4);
       x+4
    }
    
    fn maybe_add_four(y:Option<i32>) -> Option<i32> {
        y.map(add_four)
    }
    
    fn maybe_add_four_v2(y:Option<i32>)-> Option<i32> {
        
        let  k = y.map(|x| x+4);
        
       println!("{:?}",k) ;
       y.map(|x| x+4)
        
    }
    