use std::collections::HashMap;
use std::any::TypeId;

pub struct DippotamusContainer {
    registry: HashMap<TypeId, Box<dyn Fn() -> Box<dyn std::any::Any>>>,
}

impl DippotamusContainer {
    // Create a new container
    pub fn new() -> Self {
        DippotamusContainer {
            registry: HashMap::new(),
        }
    }

    // Register a type with its implementation
    pub fn register<T: 'static, F: Fn() -> T + 'static>(&mut self, factory: F) {
        self.registry.insert(TypeId::of::<T>(), Box::new(move || Box::new(factory())));
    }

    // Resolve a type
    pub fn resolve<T: 'static>(&self) -> Option<Box<T>> {
        self.registry.get(&TypeId::of::<T>()).map(|factory| factory().downcast::<T>().ok().unwrap())
    }
}