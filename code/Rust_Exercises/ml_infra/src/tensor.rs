use std::fmt;

#[warn(clippy::pedantic)]
#[derive (Debug, PartialEq)]
struct Tensor{
    matrix: Vec<f32>,
    row: usize,
    col: usize
}

trait Multiply{
    fn mult(&self, other: &Tensor)->Result<Tensor, String>;
}

impl Multiply for Tensor{
    fn mult(&self, other:&Tensor)->Result<Tensor, String>{
       match  self.matmul(&other){
        Ok(tensor)=> Ok(tensor),
        Err(e)=> Err(e)
       }
    }    
}

impl Tensor{
    fn new(matrix: Vec<f32>, row: usize, col: usize) -> Self {
        assert_eq!(matrix.len(), row*col, "Tensors has invalid shape");
        //let data_size= Self::get_shape(&matrix);
        //assert_eq!(data_size, expected_size, "Data size does not match shape dimensions.");
       Tensor {
            matrix,
            row,
            col
        }
    }

    fn validate_matrix(&self, other: &Tensor)->bool{
        self.col==other.row
    }

    fn get_index(row: usize, col:usize, m:usize)->usize{
        row*m+col
    }
    fn matmul(&self, other: &Tensor)->Result<Tensor, String>{
        // check if matrices have valid dimensions for multiplication. 
        if self.validate_matrix(other){   
            let mut result= vec![0.0; self.row*other.col];

            for i in 0..self.row{
                for j in  0..other.col{
                    for k in 0..self.col{
                        result[Self::get_index(i, j, other.col)]+= self.matrix[Self::get_index(i, k, self.col)] * other.matrix[Self::get_index(k, j, other.col)];
                    }
                }
            }

            return Ok(Tensor::new(
                result, 
                self.row,
                other.col
            ));
    
        }

        Err("Invalid dimensions cannot perform multiplication".to_string())
    }

    // lets first implement transpose
    fn transpose(&self)->Tensor{
        /*
        1. first define a new vector. 
        2. apply transpose logic 
        3. return the new vector. 
         */
    
        // ensure the vector is in 2d
        //assert_eq!(self.shape.len(), 2, "Transpose only possible for 2D matrix");


        let mut transposed_matrix= vec![0.0; self.col*self.row];

        for i in 0..self.row{
            for j in  0..self.col{
                transposed_matrix[Self::get_index(j, i, self.row)]= self.matrix[Self::get_index(i, j, self.col)];
            }
        }
        
        Tensor::new(transposed_matrix, self.col, self.row)


    }

}

impl fmt::Display for Tensor{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        write!(f, "Tensor(shape: {:?}, matrix: {:?})", (self.row, self.col), self.matrix)
    }
}

fn main() {
    // Define the shape of the tensor (2 rows, 3 columns)
    // Define the data as a flattened vector
    let data = vec![2.0, 3.0, 0.3, 1.3, 1.33, 2.67];
    

    // defining an identity matrix
    let identity= vec![1.33, 0.02, 0.03, 1.20, 2.2, 3.3];

    // Create the tensor
    let tensor = Tensor::new(data, 3, 2);
    let identity_tensor= Tensor::new(identity, 2, 3);

    //println!("{}", tensor);
    /* 
    let mult_= tensor.multiply(identity_tensor);
    println!("{}", mult_);
     */

    println!("{}", tensor);
    println!("{}", tensor.transpose());
    println!("{}", identity_tensor);

    
    let result= Tensor::mult(&tensor, &identity_tensor);
    match result{
        Ok(value)=> println!("{}", value),
        Err(e)=> println!("{}", e)
    }
    
}
