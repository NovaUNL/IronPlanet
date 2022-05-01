#![deny(
    nonstandard_style,
    warnings,
    unused,
    future_incompatible,
    clippy::all,
    clippy::pedantic
)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::wildcard_imports)]

use crate::errors::Error;
use crate::keys::*;
use crate::network::cache::ClientCache;
use crate::network::endpoints::{
    AuthenticatedSupernova, BaseSupernova, UPSTREAM,
};
use crate::network::models::{self as nmodels, AuthToken};
use crate::nmodels::ClientMeta;
use crate::utils::get_client_meta;

use std::sync::{Arc, RwLock};

pub(crate) mod coersion;
pub mod errors;
pub mod keys;
pub mod models;
mod network;
mod utils;

const DEFAULT_PAGE_ITEM_LIMIT: u16 = 100;

#[derive(Default)]
pub struct Supernova {
    base: BaseSupernova,
    authenticated: AuthenticatedSupernova,
    cache: RwLock<ClientCache>,
}

#[derive(Default)]
pub struct RequestConfig {
    pub evade_cache: bool,
}

impl Supernova {
    #[must_use]
    pub fn new() -> Arc<Supernova> {
        Arc::new(Supernova::default())
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthToken, Error> {
        let creds = nmodels::BasicAuthCredentials::new(
            username,
            password,
            Some(get_client_meta()),
        );
        let token = self.base.login(&creds).await?;
        self.authenticated.set_token(token.token.clone());
        Ok(token.token)
    }

    pub async fn logout(&self) -> Result<(), Error> {
        self.authenticated.logout().await?;
        self.authenticated.clear_token();
        Ok(())
    }

    pub async fn set_auth_token(&self, token: AuthToken) -> Result<(), Error> {
        self.base.verify(token.clone()).await?;
        self.authenticated.set_token(token);
        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated
            .credentials
            .lock()
            .unwrap()
            .borrow()
            .is_some()
    }

    pub async fn get_departments(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Vec<models::Department>, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if cache.departments_populated {
                return Ok(cache
                    .departments
                    .values()
                    .map(|net_department| net_department.link(self.clone()))
                    .collect::<Vec<models::Department>>());
            }
        } // Drop read lock

        let net_departments = self.base.fetch_departments().await?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.departments_populated = true;
            for net_department in net_departments {
                cache.departments.insert(net_department.id, net_department);
            }
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .departments
                .values()
                .map(|net_department| net_department.link(self.clone()))
                .collect())
        }
    }

    pub async fn get_buildings(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Vec<models::Building>, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if cache.buildings_populated {
                return Ok(cache
                    .buildings
                    .values()
                    .map(|building| building.link(&self.clone()))
                    .collect::<Vec<models::Building>>());
            }
        } // Drop read lock

        let net_buildings = self.base.fetch_buildings().await?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.buildings_populated = true;
            for net_building in net_buildings {
                cache.buildings.insert(net_building.id, net_building);
            }
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .buildings
                .values()
                .map(|net_building| net_building.link(&self.clone()))
                .collect())
        }
    }

    pub async fn get_places(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Vec<models::Place>, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if cache.places_populated {
                return Ok(cache
                    .places
                    .values()
                    .map(|net_place| net_place.link(self.clone()))
                    .collect::<Vec<models::Place>>());
            }
        }
        // Drop read lock
        let net_places = self.base.fetch_places().await?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.places_populated = true;
            for net_place in net_places {
                cache.places.insert(net_place.id, net_place);
            }
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .places
                .values()
                .map(|net_place| net_place.link(self.clone()))
                .collect())
        }
    }

    pub async fn get_classes(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Vec<models::Class>, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if cache.classes_populated {
                return Ok(cache
                    .classes
                    .values()
                    .map(|net_class| net_class.link(&self.clone()))
                    .collect::<Vec<models::Class>>());
            }
        } // Drop read lock

        let net_classes = self.base.fetch_classes().await?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.classes_populated = true;
            for net_class in net_classes {
                cache.classes.insert(net_class.id, net_class);
            }
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .classes
                .values()
                .map(|net_building| net_building.link(&self.clone()))
                .collect())
        }
    }

    pub async fn get_courses(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Vec<models::Course>, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if cache.courses_populated {
                return Ok(cache
                    .courses
                    .values()
                    .map(|net_course| net_course.link(self.clone()))
                    .collect::<Vec<models::Course>>());
            }
        } // Drop read lock

        let net_courses = self.base.fetch_courses().await?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.courses_populated = true;
            for ncourse in net_courses {
                cache.courses.insert(ncourse.id, ncourse);
            }
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .courses
                .values()
                .map(|net_course| net_course.link(self.clone()))
                .collect())
        }
    }

    pub async fn get_building(
        self: &Arc<Supernova>,
        id: keys::BuildingKey,
        conf: &RequestConfig,
    ) -> Result<models::Building, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_building) = cache.buildings.get(&id) {
                return Ok(net_building.link(&self.clone()));
            }
        } // Drop read lock

        let net_building = self.base.fetch_building(id).await?;

        let mut cache = self.cache.write().unwrap();
        let building = net_building.link(&self.clone());
        cache.buildings.insert(net_building.id, net_building);
        Ok(building)
    }

    pub async fn get_place(
        self: &Arc<Supernova>,
        id: keys::PlaceKey,
        conf: &RequestConfig,
    ) -> Result<models::Place, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_place) = cache.places.get(&id) {
                return Ok(net_place.link(self.clone()));
            }
        } // Drop read lock

        let net_place = self.base.fetch_place(id).await?;

        let mut cache = self.cache.write().unwrap();
        let place = net_place.link(self.clone());
        cache.places.insert(net_place.id, net_place);
        Ok(place)
    }

    pub async fn get_department(
        self: &Arc<Supernova>,
        id: keys::DepartmentKey,
        conf: &RequestConfig,
    ) -> Result<models::Department, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_department) = cache.departments.get(&id) {
                return Ok(net_department.link(self.clone()));
            }
        } // Drop read lock

        let net_department = self.base.fetch_department(id).await?;

        let mut cache = self.cache.write().unwrap();
        let department = net_department.link(self.clone());
        cache.departments.insert(net_department.id, net_department);
        Ok(department)
    }

    pub async fn get_course(
        self: &Arc<Supernova>,
        id: keys::CourseKey,
        conf: &RequestConfig,
    ) -> Result<models::Course, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_course) = cache.courses.get(&id) {
                return Ok(net_course.link(self.clone()));
            }
        } // Drop read lock

        let net_course = self.base.fetch_course(id).await?;

        let mut cache = self.cache.write().unwrap();
        let course = net_course.link(self.clone());
        cache.courses.insert(net_course.id, net_course);
        Ok(course)
    }

    pub async fn get_class(
        self: &Arc<Supernova>,
        id: keys::ClassKey,
        conf: &RequestConfig,
    ) -> Result<models::Class, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_class) = cache.classes.get(&id) {
                return Ok(net_class.link(&self.clone()));
            }
        } // Drop read lock

        let net_class = self.base.fetch_class(id).await?;

        let mut cache = self.cache.write().unwrap();
        let klass = net_class.link(&self.clone());
        cache.classes.insert(net_class.id, net_class);
        Ok(klass)
    }

    pub async fn get_class_instance(
        self: &Arc<Supernova>,
        id: keys::ClassInstanceKey,
        conf: &RequestConfig,
    ) -> Result<models::ClassInstance, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_class_inst) = cache.class_instances.get(&id) {
                return Ok(net_class_inst.link(self.clone()));
            }
        } // Drop read lock

        let net_class_inst =
            self.authenticated.fetch_class_instance(id).await?;

        let mut cache = self.cache.write().unwrap();
        net_class_inst
            .enrollments
            .iter()
            .for_each(|net_enrollment| {
                cache
                    .enrollments
                    .insert(net_enrollment.id, net_enrollment.clone());
            });
        net_class_inst.shifts.iter().for_each(|net_shift| {
            cache.class_shifts.insert(net_shift.id, net_shift.clone());
        });
        let class_inst = net_class_inst.link(self.clone());

        Ok(class_inst)
    }

    pub async fn get_student(
        self: &Arc<Supernova>,
        id: keys::StudentKey,
        conf: &RequestConfig,
    ) -> Result<models::Student, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_student) = cache.students.get(&id) {
                return Ok(net_student.link(self.clone()));
            }
        } // Drop read lock

        let net_student = self.authenticated.fetch_student(id).await?;

        let mut cache = self.cache.write().unwrap();
        let student = net_student.link(self.clone());
        cache.students.insert(net_student.id, net_student);
        Ok(student)
    }

    pub async fn get_teacher(
        self: &Arc<Supernova>,
        id: keys::ClassKey,
        conf: &RequestConfig,
    ) -> Result<models::Teacher, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_teacher) = cache.teachers.get(&id) {
                return Ok(net_teacher.link(self.clone()));
            }
        } // Drop read lock

        let net_teacher = self.authenticated.fetch_teacher(id).await?;

        let mut cache = self.cache.write().unwrap();
        let teacher = net_teacher.link(self.clone());
        cache.teachers.insert(net_teacher.id, net_teacher);
        Ok(teacher)
    }

    pub async fn get_enrollment(
        self: &Arc<Supernova>,
        id: keys::ClassKey,
        conf: &RequestConfig,
    ) -> Result<models::Enrollment, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(nenrollment) = cache.enrollments.get(&id) {
                return Ok(nenrollment.link(self.clone()));
            }
        } // Drop read lock

        let net_enrollment = self.authenticated.fetch_enrollment(id).await?;

        let mut cache = self.cache.write().unwrap();
        let enrollment = net_enrollment.link(self.clone());
        cache.enrollments.insert(net_enrollment.id, net_enrollment);
        Ok(enrollment)
    }

    pub async fn get_shift(
        self: &Arc<Supernova>,
        id: keys::ClassKey,
        conf: &RequestConfig,
    ) -> Result<models::ClassShift, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if let Some(net_shift) = cache.class_shifts.get(&id) {
                return Ok(net_shift.link(&self.clone()));
            }
        } // Drop read lock

        let net_shift = self.authenticated.fetch_shift(id).await?;

        let mut cache = self.cache.write().unwrap();
        let shift = net_shift.link(&self.clone());
        cache.class_shifts.insert(net_shift.id, net_shift);
        Ok(shift)
    }

    pub async fn get_groups(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Vec<models::Group>, Error> {
        if !conf.evade_cache {
            // Acquire read lock
            let cache = self.cache.read().unwrap();
            if cache.groups_populated {
                return Ok(cache
                    .groups
                    .values()
                    .map(|net_group| net_group.link(self.clone()))
                    .collect::<Vec<models::Group>>());
            }
        } // Drop read lock

        let net_groups = self.base.fetch_groups().await?;
        {
            let mut cache = self.cache.write().unwrap();
            cache.groups_populated = true;
            for ngroup in net_groups {
                cache.groups.insert(ngroup.id, ngroup);
            }
        }
        // Change write lock to read lock
        {
            let cache = self.cache.read().unwrap();
            Ok(cache
                .groups
                .values()
                .map(|net_group| net_group.link(self.clone()))
                .collect())
        }
    }

    pub async fn get_group(
        self: &Arc<Supernova>,
        id: keys::GroupKey,
    ) -> Result<models::Group, Error> {
        let net_group = self.base.fetch_group(id).await?;
        Ok(net_group.link(self))
    }

    pub async fn get_events_front_page(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Option<Arc<models::EventsPage>>, Error> {
        let key = (DEFAULT_PAGE_ITEM_LIMIT, 0);
        self.get_events_page(key, conf).await
    }

    pub async fn get_events_page(
        self: &Arc<Supernova>,
        key: EventsPageKey,
        _conf: &RequestConfig,
    ) -> Result<Option<Arc<models::EventsPage>>, Error> {
        let net_events_page = self.base.fetch_events(key).await?;
        if net_events_page.results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(net_events_page.link(&self.clone(), key)))
        }
    }

    pub async fn get_event(
        self: &Arc<Supernova>,
        _key: EventKey,
        _conf: &RequestConfig,
    ) -> Result<models::Event, Error> {
        todo!()
    }

    pub async fn get_news_front_page(
        self: &Arc<Supernova>,
        conf: &RequestConfig,
    ) -> Result<Option<Arc<models::NewsPage>>, Error> {
        let key = (DEFAULT_PAGE_ITEM_LIMIT, 0);
        self.get_news_page(key, conf).await
    }

    pub async fn get_news_page(
        self: &Arc<Supernova>,
        key: NewsPageKey,
        _conf: &RequestConfig,
    ) -> Result<Option<Arc<models::NewsPage>>, Error> {
        let net_news_page = self.base.fetch_news(key).await?;
        if net_news_page.results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(net_news_page.link(&self.clone(), key)))
        }
    }

    pub async fn load_resource(
        self: &Arc<Supernova>,
        url: &str,
    ) -> Result<Vec<u8>, Error> {
        let url = format!("{}{}", *UPSTREAM, url);
        if self.is_authenticated() {
            self.authenticated.fetch_bytes(&url).await
        } else {
            self.base.fetch_bytes(&url).await
        }
    }

    pub async fn warmup(self: &Arc<Supernova>) -> Result<(), Error> {
        let conf = RequestConfig::default();
        self.get_buildings(&conf).await?;
        self.get_courses(&conf).await?;
        self.get_classes(&conf).await?;
        self.get_departments(&conf).await?;
        self.get_places(&conf).await?;
        self.get_groups(&conf).await?;
        Ok(())
    }
}
