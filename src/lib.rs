use crate::errors::Error;
use crate::keys::*;
use crate::network::models::{AuthToken, BasicAuthCredentials};
use crate::network::{NetworkClient};
use std::sync::Arc;

pub(crate) mod coersion;
mod errors;
pub mod keys;
pub mod models;
mod network;

pub struct Supernova {
    service: Arc<NetworkClient>,
}

impl Supernova {
    pub fn new() -> Supernova {
        Supernova {
            service: Arc::new(NetworkClient::default()),
        }
    }

    pub fn connect(
        self: &Arc<Supernova>,
        username: String,
        password: String,
    ) -> Result<AuthToken, Error> {
        let creds = BasicAuthCredentials::new(username, password, None);
        self.service.login(creds)
    }

    pub fn connect_token(self: &Arc<Supernova>, token: AuthToken) -> Result<(), Error> {
        self.service.verify_login(token)
    }

    pub fn get_building(&self, id: BuildingKey) -> Result<models::Building, Error> {
        self.service.fetch_building(id, self.service.clone())
    }

    pub fn get_place(&self, id: PlaceKey) -> Result<models::Place, Error> {
        self.service.fetch_place(id, self.service.clone())
    }

    pub fn get_course(&self, id: CourseKey) -> Result<models::Course, Error> {
        self.service.fetch_course(id, self.service.clone())
    }

    pub fn get_class(&self, id: ClassKey) -> Result<models::Class, Error> {
        self.service.fetch_class(id, self.service.clone())
    }

    pub fn get_class_instance(&self, id: ClassInstanceKey) -> Result<models::ClassInstance, Error> {
        self.service.fetch_class_instance(id, self.service.clone())
    }

    pub fn get_enrollment(&self, id: EnrollmentKey) -> Result<models::Enrollment, Error> {
        self.service.fetch_enrollment(id, self.service.clone())
    }

    pub fn get_shift(&self, id: ShiftKey) -> Result<models::ClassShift, Error> {
        self.service.fetch_shift(id, self.service.clone())
    }

    pub fn warmup(&self) -> Result<(), Error> {
        self.service.fetch_buildings(self.service.clone())?;
        self.service.fetch_courses(self.service.clone())?;
        self.service.fetch_classes(self.service.clone())?;
        self.service.fetch_departments(self.service.clone())?;
        self.service.fetch_places(self.service.clone())?;
        Ok(())
    }
}
