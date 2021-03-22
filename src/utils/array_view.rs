
#[derive(Debug)]
pub struct ArrayView<T> {
    pub data: *const T,
    pub size: usize
}

impl<T> ArrayView<T> {
    pub fn from_vector<VecT>(vector: &Vec<VecT>) -> ArrayView<T> {
        ArrayView::<T> {
            data: &vector[0] as *const VecT as *const T,
            size: vector.len() * (std::mem::size_of::<VecT>() / std::mem::size_of::<T>())
        }
    }
    pub fn empty(&self) -> bool {
        self.size == 0
    }
}
