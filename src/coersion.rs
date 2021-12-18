use crate::errors::Error;
use crate::keys::*;
use crate::models::*;
use crate::{RequestConfig, Supernova};
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

/// Something that can be obtained from feeding a reference to a client
pub(crate) trait CoersibleEntity<I> {
    fn coerce(id: &I, client: Arc<Supernova>) -> Result<Self, Error>
    where
        Self: Sized;
}

/// A lazily loaded reference to a `CoercibleEntity`
#[derive(Clone)]
pub(crate) struct ObjRef<T: CoersibleEntity<I>, I> {
    identifier: I,
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
    pub(crate) fn coerce(&self) -> Result<T, Error> {
        T::coerce(&self.identifier, self.client.clone())
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

impl CoersibleEntity<DepartmentKey> for Department {
    fn coerce(id: &DepartmentKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_department(*id, &RequestConfig::default())
    }
}

impl ObjRef<Department, DepartmentKey> {}

impl CoersibleEntity<BuildingKey> for Building {
    fn coerce(id: &BuildingKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_building(*id, &RequestConfig::default())
    }
}

impl ObjRef<Building, BuildingKey> {}

impl CoersibleEntity<PlaceKey> for Place {
    fn coerce(id: &PlaceKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_place(*id, &RequestConfig::default())
    }
}

impl ObjRef<Place, PlaceKey> {}

impl CoersibleEntity<CourseKey> for Course {
    fn coerce(id: &CourseKey, client: Arc<Supernova>) -> Result<Course, Error> {
        client.get_course(*id, &RequestConfig::default())
    }
}

impl ObjRef<Course, CourseKey> {}

impl CoersibleEntity<ClassKey> for Class {
    fn coerce(id: &ClassKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_class(*id, &RequestConfig::default())
    }
}

impl ObjRef<Class, ClassKey> {}

impl CoersibleEntity<ClassInstanceKey> for ClassInstance {
    fn coerce(id: &ClassInstanceKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_class_instance(*id, &RequestConfig::default())
    }
}

impl ObjRef<ClassInstance, ClassInstanceKey> {}

impl CoersibleEntity<StudentKey> for Student {
    fn coerce(id: &StudentKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_student(*id, &RequestConfig::default())
    }
}

impl ObjRef<Student, StudentKey> {}

impl CoersibleEntity<TeacherKey> for Teacher {
    fn coerce(id: &TeacherKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_teacher(*id, &RequestConfig::default())
    }
}

impl ObjRef<Teacher, TeacherKey> {}

impl CoersibleEntity<EnrollmentKey> for Enrollment {
    fn coerce(id: &EnrollmentKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_enrollment(*id, &RequestConfig::default())
    }
}

impl ObjRef<Enrollment, EnrollmentKey> {}

impl CoersibleEntity<ShiftKey> for ClassShift {
    fn coerce(id: &ShiftKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_shift(*id, &RequestConfig::default())
    }
}

impl ObjRef<ClassShift, ShiftKey> {}

impl CoersibleEntity<NewsPageKey> for Option<Arc<NewsPage>> {
    fn coerce(id: &NewsPageKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_news_page(*id, &RequestConfig::default())
    }
}

impl ObjRef<ClassShift, ShiftKey> {}
