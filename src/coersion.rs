use std::marker::PhantomData;
use std::sync::Arc;
use crate::NetworkClient;
use crate::errors::Error;
use crate::keys::*;
use crate::models::*;

/// Something that can be obtained from feeding a reference to a client
pub(crate) trait CoersibleEntity<I> {
    fn coerce(id: &I, client: Arc<NetworkClient>) -> Result<Self, Error> where Self: Sized;
}


/// A lazily loaded reference to a `CoersibleEntity`
pub(crate) struct ObjRef<T: CoersibleEntity<I>, I> {
    identifier: I,
    _type: PhantomData<T>,
    // obj: Option<T>,// OnceCell
    client: Arc<NetworkClient>,
}

impl<T: CoersibleEntity<I>, I> ObjRef<T, I> {
    pub(crate) fn new(identifier: I, client: Arc<NetworkClient>) -> ObjRef<T, I> {
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
    fn coerce(id: &DepartmentKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let client_ref = client.clone();
        client.fetch_department(*id, client_ref)
    }
}

impl ObjRef<Department, DepartmentKey> {}

impl CoersibleEntity<BuildingKey> for Building {
    fn coerce(id: &BuildingKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let client_ref = client.clone();
        client.fetch_building(*id, client_ref)
    }
}

impl ObjRef<Building, BuildingKey> {}

impl CoersibleEntity<RoomKey> for Room {
    fn coerce(id: &RoomKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let client_ref = client.clone();
        client.fetch_room(*id, client_ref)
    }
}

impl ObjRef<Room, RoomKey> {}

impl CoersibleEntity<CourseKey> for Course {
    fn coerce(id: &CourseKey, client: Arc<NetworkClient>) -> Result<Course, Error> {
        let client_ref = client.clone();
        client.fetch_course(*id, client_ref)
    }
}

impl ObjRef<Course, CourseKey> {}

impl CoersibleEntity<ClassKey> for Class {
    fn coerce(id: &ClassKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let client_ref = client.clone();
        client.fetch_class(*id, client_ref)
    }
}

impl ObjRef<Class, ClassKey> {}

impl CoersibleEntity<ClassInstanceKey> for ClassInstance {
    fn coerce(id: &ClassInstanceKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let client_ref = client.clone();
        client.fetch_class_instance(*id, client_ref)
    }
}

impl ObjRef<ClassInstance, ClassInstanceKey> {}

impl CoersibleEntity<StudentKey> for Student {
    fn coerce(id: &StudentKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let client_ref = client.clone();
        client.fetch_student(*id, client_ref)
    }
}

impl ObjRef<Student, StudentKey> {}

impl CoersibleEntity<TeacherKey> for Teacher {
    fn coerce(id: &TeacherKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let client_ref = client.clone();
        client.fetch_teacher(*id, client_ref)
    }
}


impl ObjRef<Teacher, TeacherKey> {}

impl CoersibleEntity<EnrollmentKey> for Enrollment {
    fn coerce(id: &EnrollmentKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let enrollment_ref = client.clone();
        client.fetch_enrollment(*id, enrollment_ref)
    }
}

impl ObjRef<Enrollment, EnrollmentKey> {}

impl CoersibleEntity<ShiftKey> for ClassShift {
    fn coerce(id: &ShiftKey, client: Arc<NetworkClient>) -> Result<Self, Error> {
        let enrollment_ref = client.clone();
        client.fetch_shift(*id, enrollment_ref)
    }
}

impl ObjRef<ClassShift, ShiftKey> {}