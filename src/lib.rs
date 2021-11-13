use crate::errors::Error;
use crate::keys::*;
use crate::network::cache::ClientCache;
use crate::network::endpoints::{AuthenticatedSupernova, BaseSupernova};
use crate::network::http::HTTPClient;
use crate::network::models::{self as nmodels, AuthToken};
use crate::nmodels::ClientMeta;
use crate::utils::get_client_meta;
use std::sync::{Arc, RwLock};

pub(crate) mod coersion;
mod errors;
pub mod keys;
pub mod models;
mod network;
mod utils;

#[derive(Default)]
pub struct Supernova {
    base: BaseSupernova,
    authenticated: AuthenticatedSupernova,
    http_client: HTTPClient,
    cache: RwLock<ClientCache>,
}

impl Supernova {
    pub fn new() -> Arc<Supernova> {
        Arc::new(Supernova::default())
    }

    pub fn login(&self, username: &str, password: &str) -> Result<AuthToken, Error> {
        let creds = nmodels::BasicAuthCredentials::new(username, password, get_client_meta());
        let token = self.base.login(&self.http_client, creds)?;
        self.authenticated.set_token(token.token.clone());
        Ok(token.token)
    }

    pub fn logout(&self) -> Result<(), Error> {
        self.authenticated.logout(&self.http_client)?;
        self.authenticated.clear_token();
        Ok(())
    }

    pub fn set_auth_token(&self, token: AuthToken) -> Result<(), Error> {
        self.base.verify(&self.http_client, token.clone())?;
        self.authenticated.set_token(token);
        Ok(())
    }

    pub fn get_departments(self: &Arc<Supernova>) -> Result<Vec<models::Department>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.departments_populated {
                return Ok(cache
                    .departments
                    .values()
                    .map(|ndept| ndept.link(self.clone()))
                    .collect::<Vec<models::Department>>());
            }
        }
        // Drop read lock
        let departments = self.base.fetch_departments(&self.http_client)?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.departments_populated = true;
            departments.into_iter().for_each(|ndept| {
                cache.departments.insert(ndept.id, ndept);
            });
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .departments
                .values()
                .map(|ndept| ndept.link(self.clone()))
                .collect())
        }
    }

    pub fn get_buildings(self: &Arc<Supernova>) -> Result<Vec<models::Building>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.buildings_populated {
                return Ok(cache
                    .buildings
                    .values()
                    .map(|building| building.link(self.clone()))
                    .collect::<Vec<models::Building>>());
            }
        }
        // Drop read lock
        let buildings = self.base.fetch_buildings(&self.http_client)?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.buildings_populated = true;
            buildings.into_iter().for_each(|nbuilding| {
                cache.buildings.insert(nbuilding.id, nbuilding);
            });
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .buildings
                .values()
                .map(|nbuilding| nbuilding.link(self.clone()))
                .collect())
        }
    }

    pub fn get_places(self: &Arc<Supernova>) -> Result<Vec<models::Place>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.places_populated {
                return Ok(cache
                    .places
                    .values()
                    .map(|places| places.link(self.clone()))
                    .collect::<Vec<models::Place>>());
            }
        }
        // Drop read lock
        let nplaces = self.base.fetch_places(&self.http_client)?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.places_populated = true;
            nplaces.into_iter().for_each(|nplace| {
                cache.places.insert(nplace.id, nplace);
            });
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .places
                .values()
                .map(|nplace| nplace.link(self.clone()))
                .collect())
        }
    }

    pub fn get_classes(self: &Arc<Supernova>) -> Result<Vec<models::Class>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.classes_populated {
                return Ok(cache
                    .classes
                    .values()
                    .map(|nclass| nclass.link(self.clone()))
                    .collect::<Vec<models::Class>>());
            }
        }
        // Drop read lock
        let classes = self.base.fetch_classes(&self.http_client)?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.classes_populated = true;
            classes.into_iter().for_each(|nclass| {
                cache.classes.insert(nclass.id, nclass);
            });
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .classes
                .values()
                .map(|nbuilding| nbuilding.link(self.clone()))
                .collect())
        }
    }

    pub fn get_courses(self: &Arc<Supernova>) -> Result<Vec<models::Course>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.courses_populated {
                return Ok(cache
                    .courses
                    .values()
                    .map(|ncourse| ncourse.link(self.clone()))
                    .collect::<Vec<models::Course>>());
            }
        }
        // Drop read lock
        let classes = self.base.fetch_courses(&self.http_client)?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.courses_populated = true;
            classes.into_iter().for_each(|ncourse| {
                cache.courses.insert(ncourse.id, ncourse);
            });
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .courses
                .values()
                .map(|ncourse| ncourse.link(self.clone()))
                .collect())
        }
    }

    pub fn get_building(
        self: &Arc<Supernova>,
        id: keys::BuildingKey,
    ) -> Result<models::Building, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nbuilding) = cache.buildings.get(&id) {
                return Ok(nbuilding.link(self.clone()));
            }
        }
        // Drop read lock
        let nbuilding = self.base.fetch_building(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let building = nbuilding.link(self.clone());
        cache.buildings.insert(nbuilding.id, nbuilding);
        Ok(building)
    }

    pub fn get_place(self: &Arc<Supernova>, id: keys::PlaceKey) -> Result<models::Place, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nplace) = cache.places.get(&id) {
                return Ok(nplace.link(self.clone()));
            }
        }
        // Drop read lock
        let nplace = self.base.fetch_place(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let room = nplace.link(self.clone());
        cache.places.insert(nplace.id, nplace);
        Ok(room)
    }

    pub fn get_department(
        self: &Arc<Supernova>,
        id: keys::DepartmentKey,
    ) -> Result<models::Department, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(ndepartment) = cache.departments.get(&id) {
                return Ok(ndepartment.link(self.clone()));
            }
        }
        // Drop read lock
        let ndepartment = self.base.fetch_department(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let department = ndepartment.link(self.clone());
        cache.departments.insert(ndepartment.id, ndepartment);
        Ok(department)
    }

    pub fn get_course(self: &Arc<Supernova>, id: keys::CourseKey) -> Result<models::Course, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(ncourse) = cache.courses.get(&id) {
                return Ok(ncourse.link(self.clone()));
            }
        }
        // Drop read lock
        let ncourse = self.base.fetch_course(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let course = ncourse.link(self.clone());
        cache.courses.insert(ncourse.id, ncourse);
        Ok(course)
    }

    pub fn get_class(self: &Arc<Supernova>, id: keys::ClassKey) -> Result<models::Class, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nclass) = cache.classes.get(&id) {
                return Ok(nclass.link(self.clone()));
            }
        }
        // Drop read lock
        let nclass = self.base.fetch_class(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let klass = nclass.link(self.clone());
        cache.classes.insert(nclass.id, nclass);
        Ok(klass)
    }

    pub fn get_class_instance(
        self: &Arc<Supernova>,
        id: keys::ClassInstanceKey,
    ) -> Result<models::ClassInstance, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nclass_inst) = cache.class_instances.get(&id) {
                return Ok(nclass_inst.link(self.clone()));
            }
        }
        // Drop read lock
        let nclass_inst = self
            .authenticated
            .fetch_class_instance(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        nclass_inst.enrollments.iter().for_each(|nenrollment| {
            cache.enrollments.insert(nenrollment.id, nenrollment.clone());
        });
        nclass_inst.shifts.iter().for_each(|nshift| {
            cache.class_shifts.insert(nshift.id, nshift.clone());
        });
        let class_inst = nclass_inst.link(self.clone());

        Ok(class_inst)
    }

    pub fn get_student(
        self: &Arc<Supernova>,
        id: keys::StudentKey,
    ) -> Result<models::Student, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nstudent) = cache.students.get(&id) {
                return Ok(nstudent.link(self.clone()));
            }
        }
        // Drop read lock
        let nstudent = self.authenticated.fetch_student(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let student = nstudent.link(self.clone());
        cache.students.insert(nstudent.id, nstudent);
        Ok(student)
    }

    pub fn get_teacher(
        self: &Arc<Supernova>,
        id: keys::ClassKey,
    ) -> Result<models::Teacher, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nteacher) = cache.teachers.get(&id) {
                return Ok(nteacher.link(self.clone()));
            }
        }
        // Drop read lock
        let nteacher = self.authenticated.fetch_teacher(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let teacher = nteacher.link(self.clone());
        cache.teachers.insert(nteacher.id, nteacher);
        Ok(teacher)
    }

    pub fn fetch_enrollment(
        self: &Arc<Supernova>,
        id: keys::ClassKey,
    ) -> Result<models::Enrollment, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nenrollment) = cache.enrollments.get(&id) {
                return Ok(nenrollment.link(self.clone()));
            }
        }
        // Drop read lock
        let nenrollment = self.authenticated.fetch_enrollment(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let enrollment = nenrollment.link(self.clone());
        cache.enrollments.insert(nenrollment.id, nenrollment);
        Ok(enrollment)
    }

    pub fn fetch_shift(
        self: &Arc<Supernova>,
        id: keys::ClassKey,
    ) -> Result<models::ClassShift, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nshift) = cache.class_shifts.get(&id) {
                return Ok(nshift.link(self.clone()));
            }
        }
        // Drop read lock
        let nshift = self.authenticated.fetch_shift(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let shift = nshift.link(self.clone());
        cache.class_shifts.insert(nshift.id, nshift);
        Ok(shift)
    }

    pub fn warmup(self: &Arc<Supernova>) -> Result<(), Error> {
        self.get_buildings()?;
        self.get_courses()?;
        self.get_classes()?;
        self.get_departments()?;
        self.get_places()?;
        Ok(())
    }
}
