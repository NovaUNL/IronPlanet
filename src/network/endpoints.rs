use crate::keys::*;
use crate::network::{http::*, models as nmodels};
use crate::AuthToken;
use crate::{keys, Error};

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env;
use std::fmt;
use std::sync::Mutex;

use hyper::Method;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;

lazy_static! {
    pub(crate) static ref UPSTREAM: String = {
        let default = "https://supernova.nunl.pt";
        // Allowing this override in release builds would allow token hijacking
        if cfg!(debug_assertions){
            if let Ok(url) = env::var("SUPERNOVA_UPSTREAM") {
                url
            } else {
                default.to_string()
            }
        } else {
            default.to_string()
        }
    };
}

#[allow(dead_code)]
pub enum Endpoint {
    Login,
    Logout,
    TokenValidation,

    Profile(UserKey),

    Buildings,
    Building(BuildingKey),
    Places,
    Place(RoomKey),
    Departments,
    Department(DepartmentKey),
    Courses,
    Course(CourseKey),
    Classes,
    Class(ClassKey),
    ClassInstance(ClassInstanceKey),
    Student(StudentKey),
    Teacher(TeacherKey),
    Enrollment(EnrollmentKey),
    Shift(ShiftKey),

    Groups,
    Group(GroupKey),
    EventsPage(EventsPageKey),

    NewsItemPage(NewsPageKey),

    LearningAreas,
    LearningArea(LearningAreaKey),
    LearningSubarea(LearningSubareaKey),
    LearningSection(LearningSectionKey),

    Question(QuestionKey),

    Transportation,
    Weather,

    Services,
    Service(ServiceKey),
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str(&UPSTREAM);
        let _ = f.write_str("/api/");
        match self {
            Endpoint::Login => f.write_str("login"),
            Endpoint::Logout => f.write_str("logout"),
            Endpoint::TokenValidation => f.write_str("validation"),

            Endpoint::Profile(id) => {
                f.write_fmt(format_args!("services/{}", id))
            }

            Endpoint::Buildings => f.write_str("buildings"),
            Endpoint::Building(id) => {
                f.write_fmt(format_args!("building/{}", id))
            }
            Endpoint::Place(id) => f.write_fmt(format_args!("place/{}", id)),
            Endpoint::Places => f.write_str("places"),
            Endpoint::Departments => f.write_str("departments"),
            Endpoint::Department(id) => {
                f.write_fmt(format_args!("department/{}", id))
            }
            Endpoint::Courses => f.write_str("courses"),
            Endpoint::Course(id) => f.write_fmt(format_args!("course/{}", id)),
            Endpoint::Classes => f.write_str("classes"),
            Endpoint::Class(id) => f.write_fmt(format_args!("class/{}", id)),
            Endpoint::ClassInstance(id) => {
                f.write_fmt(format_args!("class/i/{}", id))
            }
            Endpoint::Student(id) => {
                f.write_fmt(format_args!("student/{}", id))
            }
            Endpoint::Teacher(id) => {
                f.write_fmt(format_args!("teacher/{}", id))
            }
            Endpoint::Enrollment(id) => {
                f.write_fmt(format_args!("enrollment/{}", id))
            }
            Endpoint::Shift(id) => f.write_fmt(format_args!("shift/{}", id)),

            Endpoint::Groups => f.write_str("groups"),
            Endpoint::Group(id) => f.write_fmt(format_args!("group/{}", id)),
            Endpoint::EventsPage((limit, offset)) => f.write_fmt(format_args!(
                "events?limit={}&offset={}",
                limit, offset
            )),

            Endpoint::NewsItemPage((limit, offset)) => f.write_fmt(
                format_args!("news?limit={}&offset={}", limit, offset),
            ),

            Endpoint::LearningAreas => f.write_str("learning/areas"),
            Endpoint::LearningArea(id) => {
                f.write_fmt(format_args!("learning/area/{}", id))
            }
            Endpoint::LearningSubarea(id) => {
                f.write_fmt(format_args!("learning/subarea/{}", id))
            }
            Endpoint::LearningSection(id) => {
                f.write_fmt(format_args!("learning/section/{}", id))
            }
            Endpoint::Question(id) => {
                f.write_fmt(format_args!("learning/question/{}", id))
            }

            Endpoint::Transportation => f.write_str("transportation/day"),
            Endpoint::Weather => f.write_str("weather"),
            Endpoint::Services => f.write_str("services"),
            Endpoint::Service(id) => {
                f.write_fmt(format_args!("services/{}", id))
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct BaseSupernova {}

impl BaseSupernova {
    #[allow(clippy::unused_self)]
    async fn generic_fetch<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, Error> {
        let response = Request::new(&url).send().await?;
        response.deserialize().await
    }

    #[allow(clippy::unused_self)]
    pub(crate) async fn fetch_bytes(
        &self,
        url: &str,
    ) -> Result<Vec<u8>, Error> {
        let response = if url.starts_with('/') {
            let url = format!("{}{}", *UPSTREAM, url);
            Request::new(&url)
        } else {
            Request::new(url)
        }
        .send()
        .await?;
        response.to_vec().await
    }

    #[allow(clippy::unused_self)]
    pub(crate) async fn login<'creds>(
        &self,
        credentials: &nmodels::BasicAuthCredentials<'creds>,
    ) -> Result<nmodels::TokenResult, Error> {
        let response = Request::new(&Endpoint::Login.to_string())
            .send_serializable(credentials)
            .await?;

        response.deserialize().await
    }

    #[allow(clippy::unused_self)]
    pub(crate) async fn verify(&self, token: AuthToken) -> Result<(), Error> {
        let response = Request::new(&Endpoint::TokenValidation.to_string())
            .attach_token(&token)
            .send()
            .await?;

        let response_text: String = response.deserialize().await?;
        if response_text == "Success" {
            Ok(())
        } else {
            Err(Error::Authentication)
        }
    }

    pub(crate) async fn fetch_departments(
        &self,
    ) -> Result<Vec<nmodels::Department>, Error> {
        self.generic_fetch(&Endpoint::Departments.to_string()).await
    }

    pub(crate) async fn fetch_buildings(
        &self,
    ) -> Result<Vec<nmodels::Building>, Error> {
        self.generic_fetch(&Endpoint::Buildings.to_string()).await
    }

    pub(crate) async fn fetch_classes(
        &self,
    ) -> Result<Vec<nmodels::Class>, Error> {
        self.generic_fetch(&Endpoint::Classes.to_string()).await
    }

    pub(crate) async fn fetch_courses(
        &self,
    ) -> Result<Vec<nmodels::Course>, Error> {
        self.generic_fetch(&Endpoint::Courses.to_string()).await
    }

    pub(crate) async fn fetch_places(
        &self,
    ) -> Result<Vec<nmodels::Place>, Error> {
        self.generic_fetch(&Endpoint::Places.to_string()).await
    }

    pub(crate) async fn fetch_building(
        &self,
        key: keys::BuildingKey,
    ) -> Result<nmodels::Building, Error> {
        self.generic_fetch(&Endpoint::Building(key).to_string())
            .await
    }

    pub(crate) async fn fetch_place(
        &self,
        key: keys::RoomKey,
    ) -> Result<nmodels::Place, Error> {
        self.generic_fetch(&Endpoint::Place(key).to_string()).await
    }

    pub(crate) async fn fetch_department(
        &self,
        key: keys::DepartmentKey,
    ) -> Result<nmodels::Department, Error> {
        self.generic_fetch(&Endpoint::Department(key).to_string())
            .await
    }

    pub(crate) async fn fetch_course(
        &self,
        key: keys::CourseKey,
    ) -> Result<nmodels::Course, Error> {
        self.generic_fetch(&Endpoint::Course(key).to_string()).await
    }

    pub(crate) async fn fetch_class(
        &self,
        key: keys::ClassKey,
    ) -> Result<nmodels::Class, Error> {
        self.generic_fetch(&Endpoint::Class(key).to_string()).await
    }

    pub(crate) async fn fetch_groups(
        &self,
    ) -> Result<Vec<nmodels::WeakGroup>, Error> {
        let endpoint = Endpoint::Groups;
        self.generic_fetch(&endpoint.to_string()).await
    }

    pub(crate) async fn fetch_group(
        &self,
        key: GroupKey,
    ) -> Result<nmodels::Group, Error> {
        let endpoint = Endpoint::Group(key);
        self.generic_fetch(&endpoint.to_string()).await
    }

    pub(crate) async fn fetch_events(
        &self,
        key: keys::EventsPageKey,
    ) -> Result<nmodels::EventsPage, Error> {
        let endpoint = Endpoint::EventsPage(key);
        self.generic_fetch(&endpoint.to_string()).await
    }

    pub(crate) async fn fetch_news(
        &self,
        key: keys::NewsPageKey,
    ) -> Result<nmodels::NewsPage, Error> {
        let endpoint = Endpoint::NewsItemPage(key);
        self.generic_fetch(&endpoint.to_string()).await
    }
}

#[derive(Default)]
pub(crate) struct AuthenticatedSupernova {
    pub(crate) credentials: Mutex<RefCell<Option<AuthToken>>>,
}

impl AuthenticatedSupernova {
    pub(crate) fn set_token(&self, token: AuthToken) {
        self.credentials
            .lock()
            .unwrap()
            .borrow_mut()
            .swap(&RefCell::new(Some(token)));
    }

    pub(crate) fn clear_token(&self) {
        self.credentials
            .lock()
            .unwrap()
            .borrow_mut()
            .swap(&RefCell::new(None));
    }

    async fn generic_fetch<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, Error> {
        let token = self
            .credentials
            .lock()
            .unwrap()
            .borrow()
            .clone()
            .ok_or(Error::MissingAuthentication)?;

        let response = Request::new(&url).attach_token(&token).send().await?;

        response.deserialize().await
    }

    #[allow(clippy::unused_self)]
    pub(crate) async fn fetch_bytes(
        &self,
        url: &str,
    ) -> Result<Vec<u8>, Error> {
        if let Some(credentials) =
            self.credentials.lock().unwrap().borrow().as_ref()
        {
            let request = if url.starts_with('/') {
                let url = format!("{}{}", *UPSTREAM, url);
                Request::new(&url)
            } else {
                Request::new(url)
            };

            let response = request.attach_token(&credentials).send().await?;

            response.to_vec().await
        } else {
            Err(Error::MissingAuthentication)
        }
    }

    pub(crate) async fn logout(&self) -> Result<(), Error> {
        if let Some(credentials) =
            self.credentials.lock().unwrap().borrow().as_ref()
        {
            let response = Request::new(&Endpoint::Logout.to_string())
                .attach_token(credentials)
                .method(Method::DELETE)
                .send()
                .await?;

            let status_code = response.code();
            if status_code.is_success() {
                Ok(())
            } else if status_code.is_server_error() {
                Err(Error::Server)
            } else if status_code.is_client_error() {
                Err(Error::Client)
            } else {
                Err(Error::Generic)
            }
        } else {
            Err(Error::MissingAuthentication)
        }
    }

    pub(crate) async fn fetch_class_instance(
        &self,
        key: keys::ClassInstanceKey,
    ) -> Result<nmodels::ClassInstance, Error> {
        self.generic_fetch(&Endpoint::ClassInstance(key).to_string())
            .await
    }

    pub(crate) async fn fetch_student(
        &self,
        key: keys::StudentKey,
    ) -> Result<nmodels::Student, Error> {
        self.generic_fetch(&Endpoint::Student(key).to_string())
            .await
    }

    pub(crate) async fn fetch_teacher(
        &self,
        key: keys::TeacherKey,
    ) -> Result<nmodels::Teacher, Error> {
        self.generic_fetch(&Endpoint::Teacher(key).to_string())
            .await
    }

    pub(crate) async fn fetch_enrollment(
        &self,
        key: keys::TeacherKey,
    ) -> Result<nmodels::Enrollment, Error> {
        self.generic_fetch(&Endpoint::Enrollment(key).to_string())
            .await
    }

    pub(crate) async fn fetch_shift(
        &self,
        key: keys::TeacherKey,
    ) -> Result<nmodels::ClassShift, Error> {
        self.generic_fetch(&Endpoint::Shift(key).to_string()).await
    }
}
