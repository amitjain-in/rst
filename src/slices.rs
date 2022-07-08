
pub fn slice_sample() {
    let s = [1, 2, 3, 4, 5, 6];
    let middle = &s[2..4];//Starting from second index till 3rd
    let left = &s[..4];//starting from 0th index till 3rd
    let right = &s[3..];//starting from 3rd index till last element of array
    println!("{:?}", middle);//"This :? is used to print Debug"
    println!("{:?}", left);
    println!("{:?}", right);
}