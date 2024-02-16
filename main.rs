//use crate::constants::add;
//mod constans2;
//use crate::constants2::get_dimensions;
//use crate::constants2::get_matrices;
//fn main() {
    //let model = get_matrices();
	
//}


// Online Rust compiler to run Rust program online
// Print "Hello World!" message

//use std::time::{Duration, Instant};
mod mnist_model;
use rand::Rng;
use std::time::Instant;
const rng_nums_count :usize = 100000;

static mut relu_table :[f32;2] = [0.0f32, 0.0f32];
unsafe fn relu(x :f32) ->f32
{
    relu_table[1] = x;
    relu_table[ (x > 0.0f32) as usize ]
}
fn relu_2(x :f32) -> f32
{
    if x > 0.0f32
    {
        return x;
    }
    0.0f32
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut rng_nums :[f32; rng_nums_count] = [0.0f32; rng_nums_count];
    for i in 0..rng_nums_count
    {
        rng_nums[i] = rng.gen_range(-1000.0f32..1000.0f32);
    }
    
    let mut relu_greater_0 = 0.0f32; 
    let mut relu_less_0 = 0.0f32;
    let mut relu_0 = 0.0f32;
    
    unsafe {
        relu_greater_0 = relu(1.0f32);
        relu_less_0 = relu(-1.0f32);
        relu_0 = relu(0.0f32);
    }
    
    println!("greater: {}", &relu_greater_0);
    println!("equal 0 {}", &relu_0);
    println!("less 0 {}", &relu_less_0);

    let mut relu_result = 0.0f32;
    let start1 = Instant::now();
    unsafe{
        for i in 0..rng_nums_count
        {
            relu_result = relu(rng_nums[i]);
        }
        
    }
    let duration1 = start1.elapsed();
    let start2 = Instant::now();
    unsafe{
        for i in 0..rng_nums_count
        {
            relu_result = relu_2(rng_nums[i]);
        }
    }
    let duration2 = start2.elapsed();
    println!("Tricky relu: {:?}",duration1);
    println!("normal relu: {:?}", duration2);
}


