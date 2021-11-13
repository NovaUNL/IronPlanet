use crate::network::cache::ClientCache;
use crate::network::endpoints::*;
use crate::network::http::*;
use crate::network::models as nmodels;
use crate::{keys, Error};
use crate::{models, AuthToken};
use serde::de::DeserializeOwned;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Default)]
pub(crate) struct NetworkClient {
    http_client: HTTPClient,
    base: NetworkClientBase,
    authenticated: NetworkClientAuthenticated,
    pub(crate) cache: RwLock<ClientCache>,
}

impl NetworkClient {
    pub(crate) fn login(&self, creds: nmodels::BasicAuthCredentials) -> Result<AuthToken, Error> {
        let token = self.base.login(&self.http_client, creds)?;
        self.authenticated.set_token(token.token.clone());
        Ok(token.token)
    }

    pub(crate) fn verify_login(&self, token: AuthToken) -> Result<(), Error> {
        self.base.verify(&self.http_client, token.clone())?;
        self.authenticated.set_token(token);
        Ok(())
    }

    pub(crate) fn fetch_departments(
        &self,
        self_ref: Arc<NetworkClient>,
    ) -> Result<Vec<models::Department>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.departments_populated {
                return Ok(cache
                    .departments
                    .values()
                    .map(|ndept| ndept.link(self_ref.clone()))
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
                .map(|ndept| ndept.link(self_ref.clone()))
                .collect())
        }
    }

    pub(crate) fn fetch_buildings(
        &self,
        self_ref: Arc<NetworkClient>,
    ) -> Result<Vec<models::Building>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.buildings_populated {
                return Ok(cache
                    .buildings
                    .values()
                    .map(|building| building.link(self_ref.clone()))
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
                .map(|nbuilding| nbuilding.link(self_ref.clone()))
                .collect())
        }
    }

    pub(crate) fn fetch_places(
        &self,
        self_ref: Arc<NetworkClient>,
    ) -> Result<Vec<models::Place>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.places_populated {
                return Ok(cache
                    .places
                    .values()
                    .map(|places| places.link(self_ref.clone()))
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
                .map(|nplace| nplace.link(self_ref.clone()))
                .collect())
        }
    }

    pub(crate) fn fetch_classes(
        &self,
        self_ref: Arc<NetworkClient>,
    ) -> Result<Vec<models::Class>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.classes_populated {
                return Ok(cache
                    .classes
                    .values()
                    .map(|nclass| nclass.link(self_ref.clone()))
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
                .map(|nbuilding| nbuilding.link(self_ref.clone()))
                .collect())
        }
    }

    pub(crate) fn fetch_courses(
        &self,
        self_ref: Arc<NetworkClient>,
    ) -> Result<Vec<models::Course>, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if cache.courses_populated {
                return Ok(cache
                    .courses
                    .values()
                    .map(|ncourse| ncourse.link(self_ref.clone()))
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
                .map(|ncourse| ncourse.link(self_ref.clone()))
                .collect())
        }
    }

    pub(crate) fn fetch_building(
        &self,
        id: keys::BuildingKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Building, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nbuilding) = cache.buildings.get(&id) {
                return Ok(nbuilding.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nbuilding = self.base.fetch_building(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let building = nbuilding.link(self_ref.clone());
        cache.buildings.insert(nbuilding.id, nbuilding);
        Ok(building)
    }

    pub(crate) fn fetch_place(
        &self,
        id: keys::PlaceKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Place, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nplace) = cache.places.get(&id) {
                return Ok(nplace.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nplace = self.base.fetch_place(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let room = nplace.link(self_ref.clone());
        cache.places.insert(nplace.id, nplace);
        Ok(room)
    }

    pub(crate) fn fetch_department(
        &self,
        id: keys::DepartmentKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Department, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(ndepartment) = cache.departments.get(&id) {
                return Ok(ndepartment.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let ndepartment = self.base.fetch_department(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let department = ndepartment.link(self_ref.clone());
        cache.departments.insert(ndepartment.id, ndepartment);
        Ok(department)
    }

    pub(crate) fn fetch_course(
        &self,
        id: keys::CourseKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Course, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(ncourse) = cache.courses.get(&id) {
                return Ok(ncourse.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let ncourse = self.base.fetch_course(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let course = ncourse.link(self_ref.clone());
        cache.courses.insert(ncourse.id, ncourse);
        Ok(course)
    }

    pub(crate) fn fetch_class(
        &self,
        id: keys::ClassKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Class, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nclass) = cache.classes.get(&id) {
                return Ok(nclass.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nclass = self.base.fetch_class(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let klass = nclass.link(self_ref.clone());
        cache.classes.insert(nclass.id, nclass);
        Ok(klass)
    }

    pub(crate) fn fetch_class_instance(
        &self,
        id: keys::ClassInstanceKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::ClassInstance, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nclass_inst) = cache.class_instances.get(&id) {
                return Ok(nclass_inst.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nclass_inst = self
            .authenticated
            .fetch_class_instance(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let class_inst = nclass_inst.link(self_ref.clone());
        cache.class_instances.insert(nclass_inst.id, nclass_inst);
        Ok(class_inst)
    }

    pub(crate) fn fetch_student(
        &self,
        id: keys::StudentKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Student, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nstudent) = cache.students.get(&id) {
                return Ok(nstudent.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nstudent = self.authenticated.fetch_student(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let student = nstudent.link(self_ref.clone());
        cache.students.insert(nstudent.id, nstudent);
        Ok(student)
    }

    pub(crate) fn fetch_teacher(
        &self,
        id: keys::ClassKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Teacher, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nteacher) = cache.teachers.get(&id) {
                return Ok(nteacher.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nteacher = self.authenticated.fetch_teacher(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let teacher = nteacher.link(self_ref.clone());
        cache.teachers.insert(nteacher.id, nteacher);
        Ok(teacher)
    }

    pub(crate) fn fetch_enrollment(
        &self,
        id: keys::ClassKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::Enrollment, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nenrollment) = cache.enrollments.get(&id) {
                return Ok(nenrollment.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nenrollment = self.authenticated.fetch_enrollment(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let enrollment = nenrollment.link(self_ref.clone());
        cache.enrollments.insert(nenrollment.id, nenrollment);
        Ok(enrollment)
    }

    pub(crate) fn fetch_shift(
        &self,
        id: keys::ClassKey,
        self_ref: Arc<NetworkClient>,
    ) -> Result<models::ClassShift, Error> {
        // Acquire read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(nshift) = cache.class_shifts.get(&id) {
                return Ok(nshift.link(self_ref.clone()));
            }
        }
        // Drop read lock
        let nshift = self.authenticated.fetch_shift(&self.http_client, id)?;
        let mut cache = self.cache.write().unwrap();
        let shift = nshift.link(self_ref.clone());
        cache.class_shifts.insert(nshift.id, nshift);
        Ok(shift)
    }
}

#[derive(Default)]
struct NetworkClientBase {}

impl NetworkClientBase {
    fn generic_fetch<T: DeserializeOwned>(&self, http: &HTTPClient, url: &str) -> Result<T, Error> {
        let request = RequestBuilder::new(url).build();
        let json_str = http.send(request).map_err(|_| Error::ParsingError)?;

        Ok(serde_json::from_str(&json_str).map_err(|_| Error::NamelessError)?)
    }

    fn login(
        &self,
        http: &HTTPClient,
        credentials: nmodels::BasicAuthCredentials,
    ) -> Result<nmodels::TokenResult, Error> {
        let request = RequestBuilder::new(&Endpoint::Login.to_string())
            .set_method(Method::POST)
            .set_body(serde_json::json!(credentials))
            .build();
        let json_str = http.send(request)?;

        Ok(serde_json::from_str(&json_str).map_err(|_| Error::ParsingError)?)
    }

    fn verify(&self, http: &HTTPClient, token: AuthToken) -> Result<(), Error> {
        let request = RequestBuilder::new(&Endpoint::TokenValidation.to_string())
            .set_method(Method::POST)
            .set_body(serde_json::json!(nmodels::TokenCredentials::new(token)))
            .build();
        let json_str = http.send(request)?;

        Ok(serde_json::from_str(&json_str).map_err(|_| Error::ParsingError)?)
    }

    fn fetch_departments(&self, http: &HTTPClient) -> Result<Vec<nmodels::Department>, Error> {
        self.generic_fetch(http, &Endpoint::Departments.to_string())
    }

    fn fetch_buildings(&self, http: &HTTPClient) -> Result<Vec<nmodels::Building>, Error> {
        self.generic_fetch(http, &Endpoint::Buildings.to_string())
    }

    fn fetch_classes(&self, http: &HTTPClient) -> Result<Vec<nmodels::Class>, Error> {
        self.generic_fetch(http, &Endpoint::Classes.to_string())
    }

    fn fetch_courses(&self, http: &HTTPClient) -> Result<Vec<nmodels::Course>, Error> {
        self.generic_fetch(http, &Endpoint::Courses.to_string())
    }

    fn fetch_places(&self, http: &HTTPClient) -> Result<Vec<nmodels::Place>, Error> {
        self.generic_fetch(http, &Endpoint::Places.to_string())
    }

    fn fetch_building(
        &self,
        http: &HTTPClient,
        key: keys::BuildingKey,
    ) -> Result<nmodels::Building, Error> {
        self.generic_fetch(http, &Endpoint::Building(key).to_string())
    }

    fn fetch_place(&self, http: &HTTPClient, key: keys::RoomKey) -> Result<nmodels::Place, Error> {
        self.generic_fetch(http, &Endpoint::Place(key).to_string())
    }

    fn fetch_department(
        &self,
        http: &HTTPClient,
        key: keys::DepartmentKey,
    ) -> Result<nmodels::Department, Error> {
        self.generic_fetch(http, &Endpoint::Department(key).to_string())
    }

    fn fetch_course(
        &self,
        http: &HTTPClient,
        key: keys::CourseKey,
    ) -> Result<nmodels::Course, Error> {
        self.generic_fetch(http, &Endpoint::Course(key).to_string())
    }

    fn fetch_class(&self, http: &HTTPClient, key: keys::ClassKey) -> Result<nmodels::Class, Error> {
        self.generic_fetch(http, &Endpoint::Class(key).to_string())
    }
}

#[derive(Default)]
struct NetworkClientAuthenticated {
    credentials: Mutex<RefCell<Option<AuthToken>>>,
}

impl NetworkClientAuthenticated {
    fn set_token(&self, token: AuthToken) {
        self.credentials
            .lock()
            .unwrap()
            .borrow_mut()
            .swap(&RefCell::new(Some(token)));
    }

    fn generic_fetch<T: DeserializeOwned>(&self, http: &HTTPClient, url: &str) -> Result<T, Error> {
        let request = RequestBuilder::new(url)
            .add_header(
                "Authorization".to_string(),
                format!(
                    "Token {}",
                    self.credentials.lock().unwrap().borrow().as_ref().unwrap()
                ),
            )
            .build();
        let json_str = http.send(request).map_err(|_| Error::NamelessError)?;
        // request = request.set("Authorization", &format!("Token {}", self.credentials.as_ref().unwrap().sn_auth_token));
        Ok(serde_json::from_str(&json_str).map_err(|_| Error::NamelessError)?)
    }

    fn fetch_class_instance(
        &self,
        http: &HTTPClient,
        key: keys::ClassInstanceKey,
    ) -> Result<nmodels::ClassInstance, Error> {
        self.generic_fetch(http, &Endpoint::ClassInstance(key).to_string())
        // let request = RequestBuilder::new(&Endpoint::ClassInstance(key).to_string()).build();
        // let json_str = http.send(request).map_err(|_| Error::NamelessError)?;
        // Ok(serde_json::from_str(&json_str).map_err(|_| Error::NamelessError)?)
    }

    fn fetch_student(
        &self,
        http: &HTTPClient,
        key: keys::StudentKey,
    ) -> Result<nmodels::Student, Error> {
        self.generic_fetch(http, &Endpoint::Student(key).to_string())
    }

    fn fetch_teacher(
        &self,
        http: &HTTPClient,
        key: keys::TeacherKey,
    ) -> Result<nmodels::Teacher, Error> {
        self.generic_fetch(http, &Endpoint::Teacher(key).to_string())
    }

    fn fetch_enrollment(
        &self,
        http: &HTTPClient,
        key: keys::TeacherKey,
    ) -> Result<nmodels::Enrollment, Error> {
        self.generic_fetch(http, &Endpoint::Enrollment(key).to_string())
    }

    fn fetch_shift(
        &self,
        http: &HTTPClient,
        key: keys::TeacherKey,
    ) -> Result<nmodels::ClassShift, Error> {
        self.generic_fetch(http, &Endpoint::Shift(key).to_string())
    }
}
