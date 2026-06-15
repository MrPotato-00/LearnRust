use std::fmt;

#[derive(Debug)]
struct Tensor{
    data: Vec<f32>
}

impl Tensor{
    fn new(data: Vec<f32>)->Self{
        Tensor{
            data
        }
    }

    fn dot_product(&self, other:&Tensor)->f32{
        assert_eq!(self.data.len(), other.data.len(), "Not equal !!");
    
        let mut output_dproduct= 0.0;
        for i in 0..self.data.len(){
            let term= self.data[i] * other.data[i]; 
            output_dproduct+= term; 
        }

        output_dproduct
    }

    fn magnitude(&self)->f32{
        let mut sum=0.0;
        
        for i in 0..self.data.len(){
            sum+= self.data[i]*self.data[i];
        }

        sum.sqrt()
    }

    fn normalize(&mut self){
        let magntude= self.magnitude();
        if magntude!=0.0{            
            for i in 0..self.data.len(){
                let x= self.data[i]/magntude;
                self.data[i]= x;
            } 
        }
        else{
            println!("Cannot perform operation, magnitude is zero.");
        }
    }

    fn element_wise_mul(&self, other: &Tensor)->Tensor{
        let mut arr= Vec::new();

        for i in 0..self.data.len(){
            arr.push(self.data[i]*other.data[i]);
        }
        Tensor::new(arr)
    }
}


impl std::fmt::Display for Tensor{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        write!(f, "Tensor(matrix: {:?})", self.data)
    }
}



fn main(){
   let tensor1= Tensor::new(vec![2.09, 0.02, -0.41, 6.04]);

   let tensor2= Tensor::new(vec![-2.09, -0.02, 0.41, -6.04]);

   println!("{tensor1}");

    println!("The result of dot product: {}", tensor1.dot_product(&tensor2));
    let mut elementwise_mult= tensor1.element_wise_mul(&tensor2);
    println!("The resut of elementwise multiplication: {}", elementwise_mult);
    elementwise_mult.normalize();

    println!("The result of the normalization: {}", elementwise_mult);
}


