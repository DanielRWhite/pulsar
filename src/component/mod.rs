pub trait Component {
        fn get_archetype(&self) -> String {
                std::any::type_name::<Self>().to_string()                
        }

        fn pre_tick(&self);
        fn tick(&mut self);
        fn post_tick(&mut self, changes: Vec<Self>) where Self: Sized;
}