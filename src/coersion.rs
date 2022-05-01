use crate::errors::Error;
use crate::keys::*;
use crate::models::*;
use crate::{RequestConfig, Supernova};

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;

/// Something that can be obtained from feeding a reference to a client
#[async_trait]
pub(crate) trait CoersibleEntity<I> {
    async fn coerce(id: &I, client: Arc<Supernova>) -> Result<Self, Error>
    where
        Self: Sized;
}

/// A lazily loaded reference to a `CoercibleEntity`
#[derive(Clone)]
pub(crate) struct ObjRef<T: CoersibleEntity<I>, I> {
    pub(crate) identifier: I,
    _type: PhantomData<T>,
    // obj: Option<T>,// OnceCell
    client: Arc<Supernova>,
}

impl<T: CoersibleEntity<I>, I: fmt::Debug> ObjRef<T, I> {
    pub(crate) fn new(identifier: I, client: Arc<Supernova>) -> ObjRef<T, I> {
        ObjRef {
            identifier,
            _type: std::marker::PhantomData::default(),
            client,
        }
    }
}

impl<T: CoersibleEntity<I>, I> ObjRef<T, I> {
    pub(crate) async fn coerce(&self) -> Result<T, Error> {
        T::coerce(&self.identifier, self.client.clone()).await
    }
}

impl<T: CoersibleEntity<I>, I: fmt::Debug> fmt::Debug for ObjRef<T, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("ObjRef<{:?}>", self.identifier))
    }
}

/// #############################################################
///  Implementations (Lots and lots of copy-paste)
/// #############################################################

#[async_trait]
impl CoersibleEntity<DepartmentKey> for Department {
    async fn coerce(
        id: &DepartmentKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_department(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Department, DepartmentKey> {}

#[async_trait]
impl CoersibleEntity<BuildingKey> for Building {
    async fn coerce(
        id: &BuildingKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_building(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Building, BuildingKey> {}

#[async_trait]
impl CoersibleEntity<PlaceKey> for Place {
    async fn coerce(
        id: &PlaceKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_place(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Place, PlaceKey> {}

#[async_trait]
impl CoersibleEntity<CourseKey> for Course {
    async fn coerce(
        id: &CourseKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_course(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Course, CourseKey> {}

#[async_trait]
impl CoersibleEntity<ClassKey> for Class {
    async fn coerce(
        id: &ClassKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_class(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Class, ClassKey> {}

#[async_trait]
impl CoersibleEntity<ClassInstanceKey> for ClassInstance {
    async fn coerce(
        id: &ClassInstanceKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client
            .get_class_instance(*id, &RequestConfig::default())
            .await
    }
}

impl ObjRef<ClassInstance, ClassInstanceKey> {}

#[async_trait]
impl CoersibleEntity<StudentKey> for Student {
    async fn coerce(
        id: &StudentKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_student(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Student, StudentKey> {}

#[async_trait]
impl CoersibleEntity<TeacherKey> for Teacher {
    async fn coerce(
        id: &TeacherKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_teacher(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Teacher, TeacherKey> {}

#[async_trait]
impl CoersibleEntity<EnrollmentKey> for Enrollment {
    async fn coerce(
        id: &EnrollmentKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_enrollment(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Enrollment, EnrollmentKey> {}

#[async_trait]
impl CoersibleEntity<ShiftKey> for ClassShift {
    async fn coerce(
        id: &ShiftKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_shift(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<ClassShift, ShiftKey> {}

// ------------ Users ------------

#[async_trait]
impl CoersibleEntity<UserKey> for User {
    async fn coerce(
        _id: &UserKey,
        _client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        todo!()
        // client.get_user(*id, &RequestConfig::default())).await
    }
}

impl ObjRef<User, UserKey> {}

// ------------ Groups ------------

#[async_trait]
impl CoersibleEntity<EventKey> for Event {
    async fn coerce(
        _id: &EventKey,
        _client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        todo!()
        // client.get_event(*id, &RequestConfig::default()).await
    }
}

impl ObjRef<Event, EventKey> {}

#[async_trait]
impl CoersibleEntity<EventsPageKey> for Option<Arc<EventsPage>> {
    async fn coerce(
        id: &EventsPageKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_events_page(*id, &RequestConfig::default()).await
    }
}
// ------------ News --------------

#[async_trait]
impl CoersibleEntity<NewsPageKey> for Option<Arc<NewsPage>> {
    async fn coerce(
        id: &NewsPageKey,
        client: Arc<Supernova>,
    ) -> Result<Self, Error> {
        client.get_news_page(*id, &RequestConfig::default()).await
    }
}
