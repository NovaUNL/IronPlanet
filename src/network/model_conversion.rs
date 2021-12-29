use crate::coersion::ObjRef;
use crate::keys::*;
use crate::models::ShiftType;
use crate::network::models as nmodels;
use crate::network::models::RoomType;
use crate::nmodels::{GroupActivity, GroupEventType, GroupScheduling, GroupType, GroupVisibility};
use crate::{models, UPSTREAM};
use crate::{ShiftKey, Supernova};
use std::cell::Cell;
use std::sync::Arc;

impl nmodels::Building {
    pub(crate) fn link(&self, client: &Arc<Supernova>) -> models::Building {
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
            building: self.building.map(|building_ref| {
                ObjRef::<models::Building, BuildingKey>::new(building_ref, client.clone())
            }),
            picture: None,
            picture_cover: None,
            variant: if let Some(meta) = &self.room_meta {
                models::PlaceVariant::Room(models::Room {
                    department: meta.department.map(|ndept| {
                        ObjRef::<models::Department, DepartmentKey>::new(ndept, client)
                    }),
                    capacity: meta.capacity,
                    door_number: meta.door_number,
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
            building: self
                .building
                .map(|key| ObjRef::<models::Building, BuildingKey>::new(key, client)),
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
            department: self
                .department
                .map(|key| ObjRef::<models::Department, CourseKey>::new(key, client)),
        }
    }
}

impl nmodels::Class {
    pub(crate) fn link(&self, client: &Arc<Supernova>) -> models::Class {
        models::Class {
            id: self.id,
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            credits: self.credits,
            department: self
                .department
                .map(|key| ObjRef::<models::Department, DepartmentKey>::new(key, client.clone())),
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
            enrollments: self
                .enrollments
                .iter()
                .map(|enrollment| {
                    ObjRef::<models::Enrollment, EnrollmentKey>::new(enrollment.id, client.clone())
                })
                .collect(),
            information: self.information.upstream.clone(),
            avg_grade: self.avg_grade,
            shifts: self
                .shifts
                .iter()
                .map(|shift| ObjRef::<models::ClassShift, ShiftKey>::new(shift.id, client.clone()))
                .collect(),
            department: self
                .department
                .map(|key| ObjRef::<models::Department, DepartmentKey>::new(key, client)),
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
            course: self
                .course
                .map(|key| ObjRef::<models::Course, CourseKey>::new(key, client)),
            avg_grade: self.avg_grade,
            url: format!("{}{}", *UPSTREAM, self.url),
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
            thumb: self
                .thumb
                .as_ref()
                .map(|url| format!("{}{}", *UPSTREAM, url)),
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
            url: format!("{}{}", *UPSTREAM, self.url),
            client,
            thumb_cache: once_cell::sync::OnceCell::default(),
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
            normal_grade: self.normal_grade,
            normal_grade_date: self.normal_grade_date.clone(),
            recourse_grade: self.recourse_grade,
            recourse_grade_date: self.recourse_grade_date.clone(),
            special_grade: self.special_grade,
            special_grade_date: self.special_grade_date.clone(),
            improvement_grade: self.improvement_grade,
            improvement_grade_date: self.improvement_grade_date.clone(),
            approved: self.approved,
            grade: self.grade,
        }
    }
}

impl nmodels::ClassShift {
    pub(crate) fn link(&self, client: &Arc<Supernova>) -> models::ClassShift {
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
            room: self
                .room
                .map(|key| ObjRef::<models::Place, PlaceKey>::new(key, client)),
        }
    }
}

// ------------ Users ---------------

impl nmodels::User {
    #[allow(dead_code)] // TODO
    pub(crate) fn to_model(&self) -> models::User {
        models::User { id: 0 }
    }
}

// ------------ Groups --------------

impl nmodels::WeakGroup {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Group {
        models::Group {
            id: self.id,
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            url: format!("{}{}", *UPSTREAM, self.url),
            thumb: self
                .thumb
                .as_ref()
                .map(|url| format!("{}{}", *UPSTREAM, url)),
            group_type: self.group_type.into(),
            official: self.official,
            upgraded: Cell::new(false),
            client,
            outsiders_openness: once_cell::sync::OnceCell::default(),
            activities: once_cell::sync::OnceCell::default(),
            schedulings: once_cell::sync::OnceCell::default(),
            events: once_cell::sync::OnceCell::default(),
            thumb_cache: once_cell::sync::OnceCell::default(),
        }
    }
}

impl nmodels::Group {
    #[allow(unused_must_use)]
    pub(crate) fn link(&self, client: &Arc<Supernova>) -> models::Group {
        let group = models::Group {
            id: self.id,
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            url: format!("{}{}", *UPSTREAM, self.url),
            thumb: self
                .thumb
                .as_ref()
                .map(|url| format!("{}{}", *UPSTREAM, url)),
            group_type: self.group_type.into(),
            official: self.official,
            upgraded: Cell::new(true),
            client: client.clone(),
            outsiders_openness: once_cell::sync::OnceCell::default(),
            activities: once_cell::sync::OnceCell::default(),
            schedulings: once_cell::sync::OnceCell::default(),
            events: once_cell::sync::OnceCell::default(),
            thumb_cache: once_cell::sync::OnceCell::default(),
        };
        group.outsiders_openness.set(self.outsiders_openness.into());
        group.activities.set(
            self.activities
                .iter()
                .map(|activity| activity.link(client.clone()))
                .collect(),
        );
        group.schedulings.set(
            self.schedule_entries
                .iter()
                .map(nmodels::GroupScheduling::to_model)
                .collect(),
        );
        group.events.set(
            self.events
                .iter()
                .map(|event| event.link(client.clone()))
                .collect(),
        );
        group
    }
}
impl nmodels::GroupActivity {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::GroupActivity {
        match self {
            GroupActivity::Announcement(activity) => {
                models::GroupActivity::Announcement(activity.link(client))
            }
            GroupActivity::EventAnnouncement(activity) => {
                models::GroupActivity::EventAnnouncement(activity.link(client))
            }
            GroupActivity::GalleryUpload(activity) => {
                models::GroupActivity::GalleryUpload(activity.link(client))
            }
        }
    }
}

impl nmodels::Event {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::Event {
        models::Event {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            start_date: self.start_date,
            duration: self.duration,
            place: self
                .place
                .map(|key| ObjRef::<models::Place, PlaceKey>::new(key, client)),
            capacity: self.capacity,
            cost: self.cost,
            event_type: self.event_type.into(),
        }
    }
}

impl nmodels::GroupAnnouncement {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::GroupAnnouncement {
        models::GroupAnnouncement {
            author: ObjRef::<models::User, UserKey>::new(self.author, client),
            title: self.title.clone(),
            content: self.content.clone(),
            datetime: self.datetime,
        }
    }
}

impl nmodels::EventAnnouncement {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::EventAnnouncement {
        models::EventAnnouncement {
            author: ObjRef::<models::User, UserKey>::new(self.author, client.clone()),
            event: ObjRef::<models::Event, EventKey>::new(self.event, client),
            datetime: self.datetime,
        }
    }
}

impl nmodels::GalleryUpload {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::GalleryUpload {
        models::GalleryUpload {
            author: ObjRef::<models::User, UserKey>::new(self.author, client),
            datetime: self.datetime,
            item: models::GalleryItem {}, // TODO
        }
    }
}

impl nmodels::GalleryItem {
    #[allow(dead_code)]
    pub(crate) fn to_model(&self) -> models::GalleryItem {
        // TODO
        models::GalleryItem {}
    }
}

impl nmodels::GroupScheduling {
    pub(crate) fn to_model(&self) -> models::GroupScheduling {
        match self {
            GroupScheduling::ScheduleOnce(sheduling) => {
                models::GroupScheduling::Once(sheduling.to_model())
            }
            GroupScheduling::SchedulePeriodic(sheduling) => {
                models::GroupScheduling::Periodic(sheduling.to_model())
            }
        }
    }
}

impl nmodels::GroupSchedulingOnce {
    pub(crate) fn to_model(&self) -> models::GroupSchedulingOnce {
        models::GroupSchedulingOnce {
            title: self.title.clone(),
            datetime: self.datetime,
            duration: self.duration,
            revoked: self.revoked,
        }
    }
}

impl nmodels::GroupSchedulingPeriodic {
    pub(crate) fn to_model(&self) -> models::GroupSchedulingPeriodic {
        models::GroupSchedulingPeriodic {
            title: self.title.clone(),
            weekday: self.weekday.into(),
            time: self.time,
            start_date: self.start_date,
            end_date: self.end_date,
            duration: self.duration,
            revoked: self.revoked,
        }
    }
}

impl nmodels::EventsPage {
    pub(crate) fn link(
        &self,
        client: &Arc<Supernova>,
        key: EventsPageKey,
    ) -> Arc<models::EventsPage> {
        let (limit, offset) = key;
        let next_page_key = (limit, offset + u32::from(limit));
        Arc::new(models::EventsPage {
            previous_page: None,
            next_page: ObjRef::<Option<Arc<models::EventsPage>>, EventsPageKey>::new(
                next_page_key,
                client.clone(),
            ),
            items: self
                .results
                .iter()
                .map(|item| Arc::new(item.link(client.clone())))
                .collect(),
        })
    }
}

impl From<nmodels::GroupType> for models::GroupType {
    fn from(gtype: nmodels::GroupType) -> Self {
        match gtype {
            GroupType::Institutional => Self::Institutional,
            GroupType::Nuclei => Self::Nuclei,
            GroupType::AcademicAssociation => Self::AcademicAssociation,
            GroupType::Pedagogic => Self::Pedagogic,
            GroupType::Praxis => Self::Praxis,
            GroupType::Community => Self::Community,
        }
    }
}

impl From<nmodels::GroupVisibility> for models::GroupVisibility {
    fn from(visibility: nmodels::GroupVisibility) -> Self {
        match visibility {
            GroupVisibility::Secret => Self::Secret,
            GroupVisibility::Closed => Self::Closed,
            GroupVisibility::Request => Self::Request,
            GroupVisibility::Open => Self::Open,
        }
    }
}

impl From<nmodels::GroupEventType> for models::GroupEventType {
    fn from(event_type: nmodels::GroupEventType) -> Self {
        match event_type {
            GroupEventType::Generic => Self::Generic,
            GroupEventType::Talk => Self::Talk,
            GroupEventType::Workshop => Self::Workshop,
            GroupEventType::Party => Self::Party,
            GroupEventType::Contest => Self::Contest,
            GroupEventType::Fair => Self::Fair,
            GroupEventType::Meeting => Self::Meeting,
        }
    }
}

// ------------ News ----------------

impl nmodels::NewsPage {
    pub(crate) fn link(&self, client: &Arc<Supernova>, key: NewsPageKey) -> Arc<models::NewsPage> {
        let (limit, offset) = key;
        let next_page_key = (limit, offset + u32::from(limit));
        Arc::new(models::NewsPage {
            previous_page: None,
            next_page: ObjRef::<Option<Arc<models::NewsPage>>, NewsPageKey>::new(
                next_page_key,
                client.clone(),
            ),
            items: self
                .results
                .iter()
                .map(|item| Arc::new(item.link(client.clone())))
                .collect(),
        })
    }
}

impl nmodels::NewsItem {
    pub(crate) fn link(&self, client: Arc<Supernova>) -> models::NewsItem {
        models::NewsItem {
            id: self.id,
            title: self.title.to_string(),
            summary: self.summary.to_string(),
            datetime: self.datetime,
            thumb: self
                .thumb
                .as_ref()
                .map(|url| format!("{}{}", *UPSTREAM, url)),
            url: format!("{}{}", *UPSTREAM, self.url),
            client,
            thumb_cache: once_cell::sync::OnceCell::default(),
        }
    }
}
