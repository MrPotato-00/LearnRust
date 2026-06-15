use std::fmt::Debug;


#[derive(Debug)]
struct Tensor<T>{
    data: Vec<T>,
    shape: Vec<usize>,
    stride: Vec<usize>
}

impl<T: std::cmp::PartialEq + Clone> Tensor<T>{
    fn new(data: Vec<T>, shape: Vec<usize>)->Tensor<T>{
        //let shape_product: usize= shape.iter().product::<usize>();
        if shape.iter().product::<usize>()!=data.len(){
            panic!("The data size and shape do not match !!");
        }
        
        let mut stride= vec![0; shape.len()];
        let mut current_stride= 1;
        for i in (0..shape.len()).rev(){
            stride[i]=current_stride;
            current_stride*= shape[i];
        }
        Tensor{
            data,
            shape,
            stride
        }
    }
    
    // if reshape is happening then stride will change !!
    fn reshape(&mut self,new_shape: Vec<usize>){
        self.shape= new_shape;
        
        let mut stride= vec![0; self.shape.len()];
        let mut current_stride= 1;
        for i in (0..self.shape.len()).rev(){
            stride[i]= current_stride;
            current_stride*= self.shape[i];
        }

        self.stride= stride;
        
    }

    fn slice(&self, ranges: Vec<(usize, usize)>)->Tensor<T>{
        let mut result= Vec::new(); // this is the new data vector.
        let new_shape= ranges
            .iter()
            .map(|(start, end)| end-start)
            .collect();
        let (row_start, row_end)= ranges[0];
        let (col_start, col_end)= ranges[1];
        
        for r in row_start..row_end{
            for c in col_start..col_end{
                let flat_idx= r*self.stride[0]+c*self.stride[1];
                result.push(self.data[flat_idx].clone());
            }
        }
        let stride= self.stride.clone();
        Tensor{
            data: result,
            shape: new_shape,
            stride
        }
    }
    
    //todo!("Currently have not implemented anything, just checking that if aligned or not.");
    fn align_to(&mut self, alignment:usize){
        let ptr= self.data.as_ptr() as usize;
        let remainder= ptr%alignment;

        if remainder!=0{
            println!("Need to align");
        }
        else{
            println!("Already aligned !!");
        }
    }

    
}

impl<T: std::fmt::Debug> fmt::Display for Tensor<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        write!(f, "Tensor(shape: {:?}, stride: {:?}, matrix: {:?})", self.shape,self.stride, self.data)
    }
}



fn main(){
   let arr= vec![1.21, 2.04, 4.12, 6.75, 0.04, 1.02];
    
   let mut tensor= Tensor::new(arr, vec![2, 3]);
   println!("{}", tensor);
   
   let reshape_vector: Vec<usize>= vec![3, 2];
   tensor.reshape(reshape_vector);
   println!("{tensor}");


   let sliced_tensor= tensor.slice(vec![(1, 3), (0,2)]);

   println!("{sliced_tensor}");
}
