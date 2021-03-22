use std::fmt::Debug;

extern crate num_traits;

#[derive(Debug)]
pub struct ArrayView<T> {
    pub data: *const T,
    pub size: usize
}

impl<T: num_traits::Num + Debug> ArrayView<T> {
    pub fn from_vector<VecT>(vector: &Vec<VecT>) -> ArrayView<T> {
        ArrayView::<T> {
            data: &vector[0] as *const VecT as *const T,
            size: vector.len() * (std::mem::size_of::<VecT>() / std::mem::size_of::<T>())
        }
    }
    pub fn from_data(data: &T, size: usize) -> ArrayView<T> {
        ArrayView::<T> { data, size }
    }
    pub fn empty(&self) -> bool {
        self.size == 0
    }
    pub fn print(&self) {
        unsafe {
            let mut ptr = self.data;
            for i in 0..self.size {
                print!("{:?} ", *ptr);
                ptr = ptr.offset(1);
            }
            println!();
        }
    }
}
