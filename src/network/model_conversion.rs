use crate::coersion::ObjRef;
use crate::keys::*;
use crate::models;
use crate::models::ShiftType;
use crate::network::models as nmodels;
use crate::network::models::RoomType;
use crate::{Supernova, ShiftKey};
use std::sync::Arc;

impl nmodels::Building {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Building {
        models::Building {
            id: self.id,
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            places: self
                .places
                .iter()
                .map(|key| ObjRef::<models::Place, PlaceKey>::new(*key, client.clone()))
                .collect(),
        }
    }
}

impl nmodels::Place {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Place {
        models::Place {
            id: self.id,
            name: self.name.clone(),
            floor: 0,
            building: if let Some(building_ref) = self.building {
                Some(ObjRef::<models::Building, BuildingKey>::new(
                    building_ref,
                    client.clone(),
                ))
            } else {
                None
            },
            picture: None,
            picture_cover: None,
            variant: if let Some(meta) = &self.room_meta {
                models::PlaceVariant::Room(models::Room {
                    department: if let Some(ndept) = meta.department {
                        Some(ObjRef::<models::Department, DepartmentKey>::new(
                            ndept, client,
                        ))
                    } else {
                        None
                    },
                    capacity: meta.capacity.clone(),
                    door_number: meta.door_number.clone(),
                    room_type: models::RoomType::from(meta.room_type),
                    description: meta.description.clone(),
                    equipment: meta.equipment.clone(),
                })
            } else {
                models::PlaceVariant::Generic
            },
        }
    }
}

impl nmodels::Department {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Department {
        models::Department {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            courses: self
                .courses
                .iter()
                .map(|key| ObjRef::<models::Course, CourseKey>::new(*key, client.clone()))
                .collect(),
            building: if let Some(key) = self.building {
                Some(ObjRef::<models::Building, BuildingKey>::new(key, client))
            } else {
                None
            },
        }
    }
}

impl nmodels::Course {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Course {
        models::Course {
            id: self.id,
            abbreviation: self.abbreviation.clone(),
            name: self.name.clone(),
            degree: models::Degree::from(self.degree),
            department: if let Some(key) = self.department {
                Some(ObjRef::<models::Department, CourseKey>::new(key, client))
            } else {
                None
            },
        }
    }
}

impl nmodels::Class {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Class {
        models::Class {
            id: self.id,
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            credits: self.credits,
            department: if let Some(key) = self.department {
                Some(ObjRef::<models::Department, DepartmentKey>::new(
                    key,
                    client.clone(),
                ))
            } else {
                None
            },
            instances: self
                .instances
                .iter()
                .map(|key| {
                    ObjRef::<models::ClassInstance, ClassInstanceKey>::new(*key, client.clone())
                })
                .collect(),
        }
    }
}

impl nmodels::ClassInstance {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::ClassInstance {
        models::ClassInstance {
            id: self.id,
            year: self.year,
            period: models::Period::from(self.period),
            students: self
                .students
                .iter()
                .map(|key| ObjRef::<models::Student, StudentKey>::new(*key, client.clone()))
                .collect(),
            teachers: self
                .teachers
                .iter()
                .map(|key| ObjRef::<models::Teacher, TeacherKey>::new(*key, client.clone()))
                .collect(),
            information: self.information.upstream.clone(),
            avg_grade: self.avg_grade.clone(),
            shifts: self
                .shifts
                .iter()
                .map(|key| ObjRef::<models::ClassShift, ShiftKey>::new(*key, client.clone()))
                .collect(),
            department: if let Some(key) = self.department {
                Some(ObjRef::<models::Department, DepartmentKey>::new(
                    key, client,
                ))
            } else {
                None
            },
        }
    }
}

impl nmodels::Student {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Student {
        models::Student {
            id: self.id,
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            number: self.number,
            enrollments: self
                .enrollments
                .iter()
                .map(|key| ObjRef::<models::Enrollment, EnrollmentKey>::new(*key, client.clone()))
                .collect(),
            shifts: self
                .shifts
                .iter()
                .map(|key| ObjRef::<models::ClassShift, ShiftKey>::new(*key, client.clone()))
                .collect(),
            first_year: self.first_year,
            last_year: self.last_year,
            course: if let Some(key) = self.course {
                Some(ObjRef::<models::Course, CourseKey>::new(key, client))
            } else {
                None
            },
            avg_grade: self.avg_grade.clone(),
            url: self.url.clone(),
        }
    }
}

impl nmodels::Teacher {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Teacher {
        models::Teacher {
            id: self.id,
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            first_year: self.first_year,
            last_year: self.last_year,
            phone: self.phone.clone(),
            email: self.email.clone(),
            thumb: self.thumb.clone(),
            rank: self.rank.clone(),
            departments: self
                .departments
                .iter()
                .map(|key| ObjRef::<models::Department, DepartmentKey>::new(*key, client.clone()))
                .collect(),
            shifts: self
                .shifts
                .iter()
                .map(|key| ObjRef::<models::ClassShift, ShiftKey>::new(*key, client.clone()))
                .collect(),
            url: self.url.clone(),
        }
    }
}

impl nmodels::Enrollment {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Enrollment {
        models::Enrollment {
            id: self.id,
            class_instance: ObjRef::<models::ClassInstance, ClassInstanceKey>::new(
                self.class_instance,
                client.clone(),
            ),
            student: ObjRef::<models::Student, StudentKey>::new(self.student, client),
            attendance: self.attendance,
            attendance_date: self.attendance_date.clone(),
            normal_grade: self.normal_grade.clone(),
            normal_grade_date: self.normal_grade_date.clone(),
            recourse_grade: self.recourse_grade.clone(),
            recourse_grade_date: self.recourse_grade_date.clone(),
            special_grade: self.special_grade.clone(),
            special_grade_date: self.special_grade_date.clone(),
            improvement_grade: self.improvement_grade,
            improvement_grade_date: self.improvement_grade_date.clone(),
            approved: self.approved,
            grade: self.grade.clone(),
        }
    }
}

impl nmodels::ClassShift {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::ClassShift {
        models::ClassShift {
            id: self.id,
            number: self.number,
            shift_type: ShiftType::from(self.shift_type),
            teachers: self
                .teachers
                .iter()
                .map(|key| ObjRef::<models::Teacher, TeacherKey>::new(*key, client.clone()))
                .collect(),
            instances: self
                .instances
                .iter()
                .map(|shift_inst| shift_inst.to_model(client.clone()))
                .collect(),
        }
    }
}

impl From<nmodels::Degree> for models::Degree {
    fn from(degree: nmodels::Degree) -> Self {
        match degree {
            nmodels::Degree::BSc => models::Degree::BSc,
            nmodels::Degree::MSc => models::Degree::MSc,
            nmodels::Degree::PhD => models::Degree::PhD,
            nmodels::Degree::IntegratedMSc => models::Degree::IntegratedMSc,
            nmodels::Degree::PostGraduation => models::Degree::PostGraduation,
            nmodels::Degree::AdvancedStudies => models::Degree::AdvancedStudies,
            nmodels::Degree::PreGraduation => models::Degree::PreGraduation,
        }
    }
}

impl From<nmodels::Period> for models::Period {
    fn from(period: nmodels::Period) -> Self {
        match period {
            nmodels::Period::Year => models::Period::Year,
            nmodels::Period::FirstSemester => models::Period::FirstSemester,
            nmodels::Period::SecondSemester => models::Period::SecondSemester,
            nmodels::Period::FirstTrimester => models::Period::FirstTrimester,
            nmodels::Period::SecondTrimester => models::Period::SecondTrimester,
            nmodels::Period::ThirdTrimester => models::Period::ThirdTrimester,
            nmodels::Period::FourthTrimester => models::Period::FourthTrimester,
        }
    }
}

impl From<nmodels::Weekday> for models::Weekday {
    fn from(weekday: nmodels::Weekday) -> Self {
        match weekday {
            nmodels::Weekday::Monday => models::Weekday::Monday,
            nmodels::Weekday::Thursday => models::Weekday::Thursday,
            nmodels::Weekday::Wednesday => models::Weekday::Wednesday,
            nmodels::Weekday::Tuesday => models::Weekday::Tuesday,
            nmodels::Weekday::Friday => models::Weekday::Friday,
            nmodels::Weekday::Saturday => models::Weekday::Saturday,
            nmodels::Weekday::Sunday => models::Weekday::Sunday,
        }
    }
}

impl From<nmodels::RoomType> for models::RoomType {
    fn from(room_type: nmodels::RoomType) -> Self {
        match room_type {
            nmodels::RoomType::Generic => models::RoomType::Generic,
            RoomType::Classroom => models::RoomType::Classroom,
            RoomType::Auditorium => models::RoomType::Auditorium,
            RoomType::Laboratory => models::RoomType::Laboratory,
            RoomType::Computer => models::RoomType::Computer,
            RoomType::Meeting => models::RoomType::Meeting,
            RoomType::Masters => models::RoomType::Masters,
            RoomType::Cabinet => models::RoomType::Cabinet,
        }
    }
}

impl From<nmodels::ShiftType> for models::ShiftType {
    fn from(shift_type: nmodels::ShiftType) -> Self {
        match shift_type {
            nmodels::ShiftType::Theoretical => models::ShiftType::Theoretical,
            nmodels::ShiftType::Practical => models::ShiftType::Practical,
            nmodels::ShiftType::PracticalTheoretical => models::ShiftType::PracticalTheoretical,
            nmodels::ShiftType::Seminar => models::ShiftType::Seminar,
            nmodels::ShiftType::TutorialOrientation => models::ShiftType::TutorialOrientation,
            nmodels::ShiftType::FieldWork => models::ShiftType::FieldWork,
            nmodels::ShiftType::OnlineTheoretical => models::ShiftType::OnlineTheoretical,
            nmodels::ShiftType::OnlinePractical => models::ShiftType::OnlinePractical,
            nmodels::ShiftType::OnlinePracticalTheoretical => {
                models::ShiftType::OnlinePracticalTheoretical
            }
        }
    }
}

// impl nmodels::ClassShift {
//     fn to_model(&self, client: Arc<NetworkClient>) -> models::ClassShift {
//         models::ClassShift {
//             id: self.id,
//             number: self.number,
//             shift_type: models::ShiftType::from(self.shift_type),
//             teachers: self.teachers.iter().map(|key| ObjRef::<models::Teacher, TeacherKey>::new(*key, client.clone())).collect(),
//             instances: self.instances.iter().map(|nshift_instance| nshift_instance.to_model(client.clone())).collect(),
//         }
//     }
// }

impl nmodels::ClassShiftInstance {
    fn to_model(&self, client: Arc<Supernova>) -> models::ClassShiftInstance {
        models::ClassShiftInstance {
            weekday: models::Weekday::from(self.weekday),
            start: self.start,
            duration: self.duration,
            room: if let Some(key) = self.room {
                Some(ObjRef::<models::Place, PlaceKey>::new(key, client))
            } else {
                None
            },
        }
    }
}
