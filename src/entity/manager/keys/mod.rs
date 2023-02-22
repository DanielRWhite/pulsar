use crate::component::Component;

fn type_of<T>(_: &T) -> String {
	std::any::type_name::<T>().to_string()
}

#[derive(Eq, Hash, Clone)]
pub struct EntityArchetypeKey {
	components: Vec<String>
}

impl EntityArchetypeKey {
	pub fn new(components: Vec<Box<dyn Component>>) -> EntityArchetypeKey {
		EntityArchetypeKey {
			components: components.iter().map(|component| type_of(&*component)).collect::<Vec<String>>()
		}
	}
}

impl PartialEq for EntityArchetypeKey {
	fn eq(&self, other: &Self) -> bool {
		// If the components don't match in length, return false
		if self.components.len() != other.components.len() {
			return false
		}

		// Clone other vec, this way we can remove items from it so we don't match the same thing twice
		// without touching the original vec
		let mut arr = other.components.clone();

		// Iterate on each component in the calling archetype
		for component in self.components.iter() {
			// If the component type (string) exists in the `mut arr`, remove it, so we don't match it twice
			// If the component doesn't exist, return false, since the archtype doesn't exist
			if let Some(pos) = arr.iter().position(|x| x == component) {
				arr.remove(pos);
			} else {
				return false
			}
		}

		// Return true since we know both arrays are the same length, and all items have been removed from the
		// array if the above for loop didn't exit early
		return true
	}
}

#[derive(Eq, Hash, Clone)]
pub struct EntityNamedKey {
	components: Vec<String>
}

impl EntityNamedKey {
	pub fn new(components: Vec<Box<dyn Component>>) -> EntityNamedKey {
		EntityNamedKey {
			components: components.iter().map(|component| type_of(&*component)).collect::<Vec<String>>()
		}
	}
}

impl PartialEq for EntityNamedKey {
	fn eq(&self, other: &Self) -> bool {
		for (self_component, other_component) in self.components.iter().zip(other.components.iter()) {
			if self_component != other_component {
				return false
			}
		}

		return true
	}
}