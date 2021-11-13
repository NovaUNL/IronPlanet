use std::marker::PhantomData;
use std::sync::Arc;
use crate::Supernova;
use crate::errors::Error;
use crate::keys::*;
use crate::models::*;

/// Something that can be obtained from feeding a reference to a client
pub(crate) trait CoersibleEntity<I> {
    fn coerce(id: &I, client: Arc<Supernova>) -> Result<Self, Error> where Self: Sized;
}


/// A lazily loaded reference to a `CoercibleEntity`
pub(crate) struct ObjRef<T: CoersibleEntity<I>, I> {
    identifier: I,
    _type: PhantomData<T>,
    // obj: Option<T>,// OnceCell
    client: Arc<Supernova>,
}

impl<T: CoersibleEntity<I>, I> ObjRef<T, I> {
    pub(crate) fn new(identifier: I, client: Arc<Supernova>) -> ObjRef<T, I> {
        ObjRef {
            identifier,
            _type: Default::default(),
            client,
        }
    }
}

impl<T: CoersibleEntity<I>, I> ObjRef<T, I> {
    pub(crate) fn coerce(&self) -> Result<T, Error> {
        T::coerce(&self.identifier, self.client.clone())
    }
}


/// #############################################################
///  Implementations (Lots and lots of copy-paste)
/// #############################################################

impl CoersibleEntity<DepartmentKey> for Department {
    fn coerce(id: &DepartmentKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_department(*id)
    }
}

impl ObjRef<Department, DepartmentKey> {}

impl CoersibleEntity<BuildingKey> for Building {
    fn coerce(id: &BuildingKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_building(*id)
    }
}

impl ObjRef<Building, BuildingKey> {}

impl CoersibleEntity<PlaceKey> for Place {
    fn coerce(id: &PlaceKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_place(*id)
    }
}

impl ObjRef<Place, PlaceKey> {}

impl CoersibleEntity<CourseKey> for Course {
    fn coerce(id: &CourseKey, client: Arc<Supernova>) -> Result<Course, Error> {
        client.get_course(*id)
    }
}

impl ObjRef<Course, CourseKey> {}

impl CoersibleEntity<ClassKey> for Class {
    fn coerce(id: &ClassKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_class(*id)
    }
}

impl ObjRef<Class, ClassKey> {}

impl CoersibleEntity<ClassInstanceKey> for ClassInstance {
    fn coerce(id: &ClassInstanceKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_class_instance(*id)
    }
}

impl ObjRef<ClassInstance, ClassInstanceKey> {}

impl CoersibleEntity<StudentKey> for Student {
    fn coerce(id: &StudentKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_student(*id)
    }
}

impl ObjRef<Student, StudentKey> {}

impl CoersibleEntity<TeacherKey> for Teacher {
    fn coerce(id: &TeacherKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.get_teacher(*id)
    }
}


impl ObjRef<Teacher, TeacherKey> {}

impl CoersibleEntity<EnrollmentKey> for Enrollment {
    fn coerce(id: &EnrollmentKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.fetch_enrollment(*id)
    }
}

impl ObjRef<Enrollment, EnrollmentKey> {}

impl CoersibleEntity<ShiftKey> for ClassShift {
    fn coerce(id: &ShiftKey, client: Arc<Supernova>) -> Result<Self, Error> {
        client.fetch_shift(*id)
    }
}

impl ObjRef<ClassShift, ShiftKey> {}