use crate::keys;
use crate::network::models as nmodels;
use std::collections::HashMap;

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub(crate) struct ClientCache {
    pub(crate) departments: HashMap<keys::DepartmentKey, nmodels::Department>,
    pub(crate) departments_populated: bool,
    pub(crate) courses: HashMap<keys::CourseKey, nmodels::Course>,
    pub(crate) courses_populated: bool,
    pub(crate) classes: HashMap<keys::ClassKey, nmodels::Class>,
    pub(crate) classes_populated: bool,
    pub(crate) class_instances: HashMap<keys::ClassInstanceKey, nmodels::ClassInstance>,
    pub(crate) class_shifts: HashMap<keys::ShiftKey, nmodels::ClassShift>,
    pub(crate) buildings: HashMap<keys::BuildingKey, nmodels::Building>,
    pub(crate) buildings_populated: bool,
    pub(crate) places: HashMap<keys::PlaceKey, nmodels::Place>,
    pub(crate) places_populated: bool,
    pub(crate) students: HashMap<keys::StudentKey, nmodels::Student>,
    pub(crate) teachers: HashMap<keys::TeacherKey, nmodels::Teacher>,
    pub(crate) enrollments: HashMap<keys::EnrollmentKey, nmodels::Enrollment>,

    pub(crate) groups: HashMap<keys::GroupKey, nmodels::WeakGroup>,
    pub(crate) groups_populated: bool,
}
