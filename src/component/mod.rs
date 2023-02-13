pub trait Component {
        fn get_archetype(&self) -> String {
                std::any::type_name::<Self>().to_string()                
        }
}