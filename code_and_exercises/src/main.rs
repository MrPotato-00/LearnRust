fn find_largest<T: PartialOrd + std::fmt::Debug>(list: &[T]){
    let mut largest= &list[0];

    for item in list{
        if item > largest{
            largest= item;
        }
    }
    println!("The largest item in the list is: {:?}", largest);
}

fn main(){
    let vec_in: Vec<i32>=  vec![3, -3, 7, 0, -5,10, 8, 6];
    let vec_fl: Vec<f64>= vec![0.0, -3.5,  7.1, -0.1, 10.2, 9.8];

    let vec_c: Vec<char>= vec!['a', 'x', 'd', 'e', 'm', 'z'];

    find_largest(&vec_in);
    find_largest(&vec_fl);
    find_largest(&vec_c);


}