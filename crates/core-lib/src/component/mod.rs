#[cfg(test)]
mod tests;

pub trait Component {
        // For archetype indexing
        fn identifier(&self) -> String; 
}