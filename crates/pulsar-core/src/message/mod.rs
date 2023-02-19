#[derive(Copy, Clone)]
pub struct Message<T: Sized, R: Sized>(T, R);

impl<T, R> Message<T, R>
where
        T: Copy + Clone + Send + Sized,
        R: Copy + Clone + Send + Sized
{
        pub fn get_type(&self) -> T {
                self.0
        }

        pub fn get_request(&self) -> R {
                self.1
        }
}