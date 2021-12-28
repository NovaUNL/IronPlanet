use crate::keys::*;
use crate::network::{http::*, models as nmodels};
use crate::AuthToken;
use crate::{keys, Error};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env;
use std::fmt;
use std::sync::Mutex;

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

pub enum Endpoint {
    Login,
    Logout,
    TokenValidation,
    Buildings,
    Building(BuildingKey),
    // Rooms,
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
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str(&UPSTREAM);
        let _ = f.write_str("/api/");
        match self {
            Endpoint::Login => f.write_str("login"),
            Endpoint::Logout => f.write_str("logout"),
            Endpoint::TokenValidation => f.write_str("validation"),
            Endpoint::Buildings => f.write_str("buildings"),
            Endpoint::Building(id) => f.write_fmt(format_args!("building/{}/", id)),
            Endpoint::Place(id) => f.write_fmt(format_args!("place/{}/", id)),
            Endpoint::Places => f.write_str("places"),
            Endpoint::Departments => f.write_str("departments"),
            Endpoint::Department(id) => f.write_fmt(format_args!("department/{}/", id)),
            Endpoint::Courses => f.write_str("courses"),
            Endpoint::Course(id) => f.write_fmt(format_args!("course/{}/", id)),
            Endpoint::Classes => f.write_str("classes"),
            Endpoint::Class(id) => f.write_fmt(format_args!("class/{}/", id)),
            Endpoint::ClassInstance(id) => f.write_fmt(format_args!("class/i/{}/", id)),
            Endpoint::Student(id) => f.write_fmt(format_args!("student/{}/", id)),
            Endpoint::Teacher(id) => f.write_fmt(format_args!("teacher/{}/", id)),
            Endpoint::Enrollment(id) => f.write_fmt(format_args!("enrollment/{}/", id)),
            Endpoint::Shift(id) => f.write_fmt(format_args!("shift/{}/", id)),
            // Endpoint::ClassInstanceSchedule(id) => {
            //     f.write_fmt(format_args!("class/i/{}/schedule", id))
            // }
            Endpoint::Groups => f.write_str("groups"),
            Endpoint::Group(id) => f.write_fmt(format_args!("group/{}/", id)),
            Endpoint::EventsPage((limit, offset)) => {
                f.write_fmt(format_args!("events/?limit={}&offset={}", limit, offset))
            }

            Endpoint::NewsItemPage((limit, offset)) => {
                f.write_fmt(format_args!("news/?limit={}&offset={}", limit, offset))
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct BaseSupernova {}

impl BaseSupernova {
    #[allow(clippy::unused_self)]
    fn generic_fetch<T: DeserializeOwned>(&self, http: &HTTPClient, url: &str) -> Result<T, Error> {
        let request = RequestBuilder::new(url).build();
        let json_str = http.send(request)?;

        serde_json::from_str(&json_str).map_err(|e| Error::Parsing(e, json_str.to_string()))
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn fetch_bytes(&self, http: &HTTPClient, url: &str) -> Result<Vec<u8>, Error> {
        let request = RequestBuilder::new(url).build();
        http.fetch_bytes(request)
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn login(
        &self,
        http: &HTTPClient,
        credentials: &nmodels::BasicAuthCredentials,
    ) -> Result<nmodels::TokenResult, Error> {
        let request = RequestBuilder::new(&Endpoint::Login.to_string())
            .set_method(Method::Post)
            .set_body(serde_json::json!(credentials))
            .build();
        let json_str = http.send(request)?;

        serde_json::from_str(&json_str).map_err(|e| Error::Parsing(e, json_str.to_string()))
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn verify(&self, http: &HTTPClient, token: AuthToken) -> Result<(), Error> {
        let request = RequestBuilder::new(&Endpoint::TokenValidation.to_string())
            .set_method(Method::Get)
            .add_header("Authorization".to_string(), format!("Token {}", token))
            .set_body(serde_json::json!(nmodels::TokenCredentials::new(token)))
            .build();
        http.send(request)?;
        // let json_str = http.send(request)?;
        // if json_str == "Success" {
        //     ???
        // }
        Ok(())
    }

    pub(crate) fn fetch_departments(
        &self,
        http: &HTTPClient,
    ) -> Result<Vec<nmodels::Department>, Error> {
        self.generic_fetch(http, &Endpoint::Departments.to_string())
    }

    pub(crate) fn fetch_buildings(
        &self,
        http: &HTTPClient,
    ) -> Result<Vec<nmodels::Building>, Error> {
        self.generic_fetch(http, &Endpoint::Buildings.to_string())
    }

    pub(crate) fn fetch_classes(&self, http: &HTTPClient) -> Result<Vec<nmodels::Class>, Error> {
        self.generic_fetch(http, &Endpoint::Classes.to_string())
    }

    pub(crate) fn fetch_courses(&self, http: &HTTPClient) -> Result<Vec<nmodels::Course>, Error> {
        self.generic_fetch(http, &Endpoint::Courses.to_string())
    }

    pub(crate) fn fetch_places(&self, http: &HTTPClient) -> Result<Vec<nmodels::Place>, Error> {
        self.generic_fetch(http, &Endpoint::Places.to_string())
    }

    pub(crate) fn fetch_building(
        &self,
        http: &HTTPClient,
        key: keys::BuildingKey,
    ) -> Result<nmodels::Building, Error> {
        self.generic_fetch(http, &Endpoint::Building(key).to_string())
    }

    pub(crate) fn fetch_place(
        &self,
        http: &HTTPClient,
        key: keys::RoomKey,
    ) -> Result<nmodels::Place, Error> {
        self.generic_fetch(http, &Endpoint::Place(key).to_string())
    }

    pub(crate) fn fetch_department(
        &self,
        http: &HTTPClient,
        key: keys::DepartmentKey,
    ) -> Result<nmodels::Department, Error> {
        self.generic_fetch(http, &Endpoint::Department(key).to_string())
    }

    pub(crate) fn fetch_course(
        &self,
        http: &HTTPClient,
        key: keys::CourseKey,
    ) -> Result<nmodels::Course, Error> {
        self.generic_fetch(http, &Endpoint::Course(key).to_string())
    }

    pub(crate) fn fetch_class(
        &self,
        http: &HTTPClient,
        key: keys::ClassKey,
    ) -> Result<nmodels::Class, Error> {
        self.generic_fetch(http, &Endpoint::Class(key).to_string())
    }

    pub(crate) fn fetch_groups(&self, http: &HTTPClient) -> Result<Vec<nmodels::WeakGroup>, Error> {
        let endpoint = Endpoint::Groups;
        self.generic_fetch(http, &endpoint.to_string())
    }

    pub(crate) fn fetch_group(
        &self,
        http: &HTTPClient,
        key: GroupKey,
    ) -> Result<nmodels::Group, Error> {
        let endpoint = Endpoint::Group(key);
        self.generic_fetch(http, &endpoint.to_string())
    }

    pub(crate) fn fetch_events(
        &self,
        http: &HTTPClient,
        key: keys::EventsPageKey,
    ) -> Result<nmodels::EventsPage, Error> {
        let endpoint = Endpoint::EventsPage(key);
        self.generic_fetch(http, &endpoint.to_string())
    }

    pub(crate) fn fetch_news(
        &self,
        http: &HTTPClient,
        key: keys::NewsPageKey,
    ) -> Result<nmodels::NewsPage, Error> {
        let endpoint = Endpoint::NewsItemPage(key);
        self.generic_fetch(http, &endpoint.to_string())
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

    fn generic_fetch<T: DeserializeOwned>(&self, http: &HTTPClient, url: &str) -> Result<T, Error> {
        let request = if let Some(credentials) = self.credentials.lock().unwrap().borrow().as_ref()
        {
            RequestBuilder::new(url)
                .add_header(
                    "Authorization".to_string(),
                    format!("Token {}", credentials),
                )
                .build()
        } else {
            return Err(Error::MissingAuthentication);
        };

        let json_str = http.send(request)?;
        serde_json::from_str(&json_str).map_err(|e| Error::Parsing(e, json_str.to_string()))
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn fetch_bytes(&self, http: &HTTPClient, url: &str) -> Result<Vec<u8>, Error> {
        if let Some(credentials) = self.credentials.lock().unwrap().borrow().as_ref() {
            let request = RequestBuilder::new(url)
                .add_header(
                    "Authorization".to_string(),
                    format!("Token {}", credentials),
                )
                .build();
            http.fetch_bytes(request)
        } else {
            Err(Error::MissingAuthentication)
        }
    }

    pub(crate) fn logout(&self, http: &HTTPClient) -> Result<nmodels::TokenResult, Error> {
        let request = if let Some(credentials) = self.credentials.lock().unwrap().borrow().as_ref()
        {
            RequestBuilder::new(&Endpoint::Logout.to_string())
                .add_header(
                    "Authorization".to_string(),
                    format!("Token {}", credentials),
                )
                .set_method(Method::Delete)
                .build()
        } else {
            return Err(Error::MissingAuthentication);
        };
        let json_str = http.send(request)?;

        serde_json::from_str(&json_str).map_err(|e| Error::Parsing(e, json_str.to_string()))
    }

    pub(crate) fn fetch_class_instance(
        &self,
        http: &HTTPClient,
        key: keys::ClassInstanceKey,
    ) -> Result<nmodels::ClassInstance, Error> {
        self.generic_fetch(http, &Endpoint::ClassInstance(key).to_string())
    }

    pub(crate) fn fetch_student(
        &self,
        http: &HTTPClient,
        key: keys::StudentKey,
    ) -> Result<nmodels::Student, Error> {
        self.generic_fetch(http, &Endpoint::Student(key).to_string())
    }

    pub(crate) fn fetch_teacher(
        &self,
        http: &HTTPClient,
        key: keys::TeacherKey,
    ) -> Result<nmodels::Teacher, Error> {
        self.generic_fetch(http, &Endpoint::Teacher(key).to_string())
    }

    pub(crate) fn fetch_enrollment(
        &self,
        http: &HTTPClient,
        key: keys::TeacherKey,
    ) -> Result<nmodels::Enrollment, Error> {
        self.generic_fetch(http, &Endpoint::Enrollment(key).to_string())
    }

    pub(crate) fn fetch_shift(
        &self,
        http: &HTTPClient,
        key: keys::TeacherKey,
    ) -> Result<nmodels::ClassShift, Error> {
        self.generic_fetch(http, &Endpoint::Shift(key).to_string())
    }
}
