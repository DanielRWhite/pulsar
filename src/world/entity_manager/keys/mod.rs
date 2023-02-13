use crate::component::Component;

#[derive(Eq, Hash, Clone)]
pub struct EntityArchetypeKey {
        components: Vec<String>
}


impl EntityArchetypeKey {
        pub fn new(components: Vec<Box<dyn Component>>) -> Self {
                EntityArchetypeKey {
                        components: components.iter().map(|x| x.get_archetype()).collect::<Vec<String>>()
                }
        }

        pub fn from_strings(components: Vec<String>) -> Self {
                EntityArchetypeKey { components }
        }

        pub fn get_components(&self) -> Vec<String> {
                self.components.clone()
        }

        pub fn include_in_query(&self, components: &EntityArchetypeKey) -> bool {
                let mut archetypes = self.components.clone();
                for component in components.get_components() {
                        if let Some(pos) = archetypes.iter().position(|x| x == &component) {
                                archetypes.remove(pos);
                        } else {
                                return false
                        }
                }

                return true
        }
}

impl PartialEq for EntityArchetypeKey {
        fn eq(&self, other: &Self) -> bool {
                let mut archetypes = other.components.clone();
                for component in self.components.iter() {
                        if let Some(pos) = archetypes.iter().position(|x| x == component) {
                                archetypes.remove(pos);
                        } else {
                                return false
                        }
                }

                if archetypes.len() == 0 {
                        return true
                } else {
                        return false
                }
        }
}