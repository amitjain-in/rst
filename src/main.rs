use crate::games::*;
use crate::shadow::shadowing;
use crate::data_types::*;
use crate::arrays::*;
use crate::ctrl_flow::*;
use crate::enums::{enum_sample, more_enum_sample};
use crate::funcs::*;
use crate::ownership::*;
use crate::slices::*;
use crate::structs::struct_sample;


pub mod games;
pub mod shadow;
pub mod data_types;
pub mod arrays;
pub mod funcs;
pub mod ctrl_flow;
pub mod ownership;
pub mod slices;
mod structs;
mod enums;

fn main() {
    //guessing_game();
    //shadowing();
    //learn_data_types();
    //learn_arrays();
    // learn_functions();
    // learn_ctrl();
    // learn_ownership();
    // learn_slicing();
    // learn_structs();
    learn_enums();
}

fn learn_enums() {
    enum_sample();
    more_enum_sample();
}

fn learn_structs() {
    struct_sample();
}

fn learn_slicing() {
    slice_sample();
}

fn learn_ownership() {
    // ptr();
    // pass_to_fn();
    // pass_references();
    // pass_references_and_mutate();
    // multiple_borrows();
    mixed_references();
}


fn learn_data_types() {
    type_of_defaults();
    tuples();
}

fn learn_ctrl() {
    if_cond(6);
    for_loop();
    loop_loop();
}

fn learn_arrays() {
    arrays_basic();
    test_array_bounds();
}

fn learn_functions() {
    println!("increment by 1 is {}", incr_by_1(5));
}