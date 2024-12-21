use std::collections::HashMap;
use std::sync::Mutex;

use crate::features::users::user_service::UserService;

pub trait Service {
    fn name(&self) -> String;
}

#[derive(Clone)]
pub enum ServiceType {
    UserService(UserService),
}

impl ServiceType {
    pub fn name(&self) -> String {
        match self {
            ServiceType::UserService(service) => service.name(),
        }
    }
}

pub struct ServiceProvider {
    services: Mutex<HashMap<String, ServiceType>>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self {
            services: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_service(&self, service: ServiceType) {
        let mut services = self.services.lock().unwrap();
        services.insert(service.name(), service);
    }

    pub fn get_service(&self, name: &str) -> Option<ServiceType> {
        let services = self.services.lock().unwrap();
        services.get(name).cloned()
    }
}
