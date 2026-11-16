//! SigmaOS-architecture: Software architecture model.
//!
//! Real implementation: components, layers, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub layer: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub id: String,
    pub component_id: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchitectureError {
    EmptyComponentName,
    DuplicateComponent(String),
}

impl fmt::Display for ArchitectureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponentName => write!(f, "component name must not be empty"),
            Self::DuplicateComponent(id) => write!(f, "duplicate component {:?}", id),
        }
    }
}

impl std::error::Error for ArchitectureError {}

pub struct Architecture {
    pub name: String,
    components: BTreeMap<String, Component>,
    layers: BTreeMap<String, Layer>,
}

impl Architecture {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), components: BTreeMap::new(), layers: BTreeMap::new() }
    }

    pub fn add_component(&mut self, c: Component) -> Result<(), ArchitectureError> {
        if c.name.trim().is_empty() { return Err(ArchitectureError::EmptyComponentName); }
        let id = c.id.clone();
        if self.components.contains_key(&id) {
            return Err(ArchitectureError::DuplicateComponent(id.clone()));
        }
        self.components.insert(id, c);
        Ok(())
    }

    pub fn component_count(&self) -> usize { self.components.len() }

    pub fn get_component(&self, id: &str) -> Option<&Component> { self.components.get(id) }

    pub fn add_layer(&mut self, l: Layer) {
        self.layers.insert(l.id.clone(), l);
    }

    pub fn get_layer(&self, id: &str) -> Option<&Layer> { self.layers.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Architecture::new("Architecture");
        assert_eq!(a.name, "Architecture");
        assert_eq!(a.component_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Architecture::new("Architecture");
        let component = Component { id: "c1".into(), name: "Controller".into(), layer: "Presentation".into() };
        a.add_component(component).unwrap();
        assert_eq!(a.component_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Architecture::new("Architecture");
        let component = Component { id: "x".into(), name: "".into(), layer: "x".into() };
        assert!(a.add_component(component).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Architecture::new("Architecture");
        let component = Component { id: "x".into(), name: "X".into(), layer: "x".into() };
        a.add_component(component).unwrap();
        let component2 = Component { id: "x".into(), name: "X".into(), layer: "x".into() };
        assert!(a.add_component(component2).is_err());
    }
}
