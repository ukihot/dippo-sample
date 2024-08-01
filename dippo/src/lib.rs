mod dippo_error;

use crate::dippo_error::SpitUpError;
use crate::dippo_error::StockpileError;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct DippotamusContainer {
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl DippotamusContainer {
    pub fn new() -> Self {
        DippotamusContainer {
            services: HashMap::new(),
        }
    }

    // サービスを登録する
    pub fn stockpile<T: 'static>(&mut self, service: T) -> Result<(), StockpileError> {
        let type_id = TypeId::of::<T>();
        if self.services.contains_key(&type_id) {
            return Err(StockpileError::AlreadyRegistered);
        }
        self.services.insert(type_id, Box::new(service));
        Ok(())
    }

    // サービスを解決する（依存関係を解決する）
    pub fn spit_up<T: 'static>(&self) -> Result<&T, SpitUpError> {
        let type_id = TypeId::of::<T>();
        let service = self.services.get(&type_id).ok_or(SpitUpError::NotFound)?;

        service.downcast_ref::<T>().ok_or(SpitUpError::NotFound)
    }
}

// `Any`トレイトを拡張して、型のダウンキャストをサポートする
trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
