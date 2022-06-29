use crate::games::*;
use crate::shadow::shadowing;
use crate::data_types::*;
use crate::arrays::*;
use crate::ctrl_flow::*;
use crate::funcs::*;


pub mod games;
pub mod shadow;
pub mod data_types;
pub mod arrays;
pub mod funcs;
pub mod ctrl_flow;

fn main() {
    //guessing_game();
    //shadowing();
    //learn_data_types();
    //learn_arrays();
    learn_functions();
    learn_ctrl();
}

fn learn_data_types() {
    type_of_defaults();
    tuples();
}

fn learn_ctrl() {
    if_cond(6);
    for_loop();
}

fn learn_arrays() {
    arrays_basic();
    test_array_bounds();
}

fn learn_functions() {
    println!("increment by 1 is {}", incr_by_1(5));
}