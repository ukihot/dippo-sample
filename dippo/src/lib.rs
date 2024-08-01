use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub struct DippotamusContainer {
    services: HashMap<TypeId, Box<dyn Any>>,
    registrations: HashMap<TypeId, TypeId>,
}

impl DippotamusContainer {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            registrations: HashMap::new(),
        }
    }

    pub fn register<T: 'static + Default, U: 'static + Default>(&mut self) {
        self.registrations
            .insert(TypeId::of::<T>(), TypeId::of::<U>());
        self.services
            .insert(TypeId::of::<U>(), Box::new(U::default()));
    }

    pub fn resolve<T: 'static + Clone>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(&registered_type_id) = self.registrations.get(&type_id) {
            if let Some(service) = self.services.get(&registered_type_id) {
                if let Some(service) = service.downcast_ref::<T>() {
                    return Some(Arc::new(service.clone()));
                }
            }
        }
        None
    }

    pub fn resolve_recursive<T: 'static + Clone>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(&registered_type_id) = self.registrations.get(&type_id) {
            if let Some(service) = self.services.get(&registered_type_id) {
                if let Some(service) = service.downcast_ref::<T>() {
                    return Some(Arc::new(service.clone()));
                } else {
                    // 再帰的に解決する方法
                    if let Some(service) = self.resolve_recursive::<T>() {
                        return Some(service);
                    }
                }
            }
        }
        None
    }
}
