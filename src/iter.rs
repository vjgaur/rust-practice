use std::collections::HashMap;

fn main (){
let vec = vec![0,1,3,3];
vec.iter()
.map(|x| x + 1)
.filter(|x| x > &1)
.for_each(|x| println!("{}", x));


let vec1 = vec![0,1,2,3];
for(i,v) in vec1.iter().chain(Some(42).iter()).enumerate(){
   
        println!("{}: {}", i,v);
    }
    
let vec2 = vec![0,1,2,3];
let vec_2: Vec<_> = vec.iter().map(|x| x*2).collect();
let map: HashMap<_,_> = vec.iter().map(|x| x*2).enumerate().collect();


let vec3: Vec<_> = vec![0,1,2,3];
vec.iter().for_each(|v| println!("{}",v));

for v in &vec {
        println!("{}", v);
    }


}