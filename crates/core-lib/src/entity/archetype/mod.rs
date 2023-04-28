#[cfg(test)]
mod tests;

#[derive(Hash)]
pub struct Archetype {
        pub components: Vec<String>
}

impl Archetype {
        pub fn new(components: Vec<String>) -> Archetype {
                Archetype { components }
        }

        pub fn add_component(&mut self, component: String) -> Option<()> {
                if let Some(pos) = self.components.iter().position(|comp| comp == &component) {
                        None
                } else {
                        self.components.push(component);
                        Some(())
                }
        }

        pub fn del_component(&mut self, component: &String) -> Option<()> {
                if let Some(pos) = self.components.iter().position(|comp| comp == component) {
                        self.components.swap_remove(pos);
                        Some(())
                } else {
                        None
                }
        }

        pub fn has_component(&self, component: &String) -> bool {
                self.components.iter().position(|comp| comp == component).is_some()
        }
}

impl std::default::Default for Archetype {
        fn default() -> Archetype {
                let components: Vec<String> = Vec::new();
                Archetype { components }
        }
}

impl PartialEq for Archetype {
        fn eq(&self, other: &Self) -> bool {
                if self.components.len() != other.components.len() {
                        false
                } else {
                        self.components.iter().all(|c| other.has_component(c))
                }                
        }
}

impl Eq for Archetype { }