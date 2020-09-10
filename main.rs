use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 4, 8, 7 , 9, 9, 9];
    let meaning = mean(&v);
    let mediann = median(&v);
    let moder = mode(&v);

    println!("{}", meaning);
    println!("{}", mediann);
    println!("{}", moder);

}


fn mean(v: &Vec<i32>)  -> f64 {

    let length = v.len();

    let sum : i32= v.iter().sum();

    sum as f64 / length as f64
}

fn median(v : &Vec<i32>) -> f64 {
    let length = v.len();
    let  median_value; 
    if length % 2 == 0 {
       median_value = (v[length / 2] +  v[(length / 2) - 1]) as f64 / 2.0  ;
    } else {
        median_value =  v[length / 2] as f64 ;
    }
    

    median_value

}


fn mode(v: &Vec<i32>) -> i32 {
    let mut hash = HashMap::new();

    for i in v {
        let count = hash.entry(i).or_insert(0);
        *count += 1;
    }
    let mut moder = 0;
    let mut nums = 0;
    for (key, value) in &hash {
        if *value > nums {
            nums = *value;
            moder = **key;
        }
    }
    
    moder
}