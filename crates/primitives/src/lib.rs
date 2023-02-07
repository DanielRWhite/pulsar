pub mod integer;

pub (crate) struct Primitive<T>(T);


impl<T> Primitive<T> {
        #[allow(dead_code)]
        pub fn new(value: T) -> Primitive<T> {
                Primitive(value)
        }
}