use crate::coersion::ObjRef;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use once_cell::sync::OnceCell;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::errors::Error;
use crate::keys::*;
pub use crate::network::models::{ClassInfo, ClassInfoEntry, ClassInfoSources};
use crate::Supernova;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum Weekday {
    Monday,
    Thursday,
    Wednesday,
    Tuesday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum Period {
    Year,
    FirstSemester,
    SecondSemester,
    FirstTrimester,
    SecondTrimester,
    ThirdTrimester,
    FourthTrimester,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum Degree {
    BSc,
    MSc,
    PhD,
    IntegratedMSc,
    PostGraduation,
    AdvancedStudies,
    PreGraduation,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum ShiftType {
    Theoretical,
    Practical,
    PracticalTheoretical,
    Seminar,
    TutorialOrientation,
    FieldWork,
    OnlineTheoretical,
    OnlinePractical,
    OnlinePracticalTheoretical,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum FileCategory {
    Image,
    Slides,
    Problems,
    Protocol,
    Seminar,
    Exam,
    Test,
    Support,
    Others,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum FileLicense {
    RightsReserved,
    PublicDomain,
    GPL,
    MIT,
    BSD,
    CCBy,
    CCBySa,
    CCByNc,
    CCBySaNc,
    GenericPermissive,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum FileVisibility {
    Public,
    Students,
    Enrolled,
    Nobody,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum Season {
    Normal,
    Exam,
    Special,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum ClassEventType {
    Test,
    Exam,
    Discussion,
    FieldTrip,
    ProjectAnnouncement,
    ProjectDelivery,
    AdditionalClass,
    Presentation,
    Seminar,
    Talk,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum RoomType {
    Generic,
    Classroom,
    Auditorium,
    Laboratory,
    Computer,
    Meeting,
    Masters,
    Cabinet,
}

#[derive(Debug, Clone)]
pub struct Department {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub(crate) courses: Vec<ObjRef<Course, CourseKey>>,
    pub(crate) building: Option<ObjRef<Building, BuildingKey>>,
}

#[derive(Debug, Clone)]
pub struct Building {
    pub id: u32,
    pub name: String,
    pub abbreviation: String,
    pub(crate) places: Vec<ObjRef<Place, PlaceKey>>,
}

#[derive(Debug, Clone)]
pub struct Place {
    pub id: PlaceKey,
    pub variant: PlaceVariant,
    pub name: String,
    pub floor: i8,
    pub(crate) building: Option<ObjRef<Building, BuildingKey>>,
    pub picture: Option<String>,
    pub picture_cover: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PlaceVariant {
    Generic,
    Room(Room),
}

#[derive(Debug, Clone)]
pub struct Room {
    pub(crate) department: Option<ObjRef<Department, DepartmentKey>>,
    pub capacity: Option<u16>,
    pub door_number: Option<u16>,
    pub room_type: RoomType,
    pub description: Option<String>,
    pub equipment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub id: u32,
    pub abbreviation: String,
    pub name: String,
    pub degree: Degree,
    pub(crate) department: Option<ObjRef<Department, DepartmentKey>>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub id: u32,
    pub name: String,
    pub abbreviation: String,
    pub credits: u32,
    pub(crate) department: Option<ObjRef<Department, DepartmentKey>>,
    pub(crate) instances: Vec<ObjRef<ClassInstance, ClassInstanceKey>>,
}

#[derive(Debug, Clone)]
pub struct ClassInstance {
    pub id: u32,
    pub year: u32,
    pub period: Period,
    pub(crate) enrollments: Vec<ObjRef<Enrollment, EnrollmentKey>>,
    pub information: Option<ClassInfo>,
    pub avg_grade: Option<f32>,
    pub(crate) shifts: Vec<ObjRef<ClassShift, ShiftKey>>,
    pub(crate) department: Option<ObjRef<Department, DepartmentKey>>,
}

// pub struct ClassInfo {
//     pub program: ClassInfoEntry,
//     pub assistance: ClassInfoEntry,
//     pub extra_info: ClassInfoEntry,
//     pub objectives: ClassInfoEntry,
//     pub competences: ClassInfoEntry,
//     pub description: ClassInfoEntry,
//     pub bibliography: ClassInfoEntry,
//     pub requirements: ClassInfoEntry,
//     pub teaching_methods: ClassInfoEntry,
//     pub evaluation_methods: ClassInfoEntry,
// }
//
//
// pub struct ClassInfoEntry {
//     pub pt: Option<String>,
//     pub en: Option<String>,
//     pub time: Option<String>,
//     pub editor: Option<String>,
// }

#[derive(Debug, Clone)]
pub struct ClassShift {
    pub id: u32,
    pub number: u16,
    pub shift_type: ShiftType,
    pub(crate) teachers: Vec<ObjRef<Teacher, TeacherKey>>,
    pub instances: Vec<ClassShiftInstance>,
}

#[derive(Debug, Clone)]
pub struct ClassShiftInstance {
    pub weekday: Weekday,
    pub start: u16,
    pub duration: u16,
    pub(crate) room: Option<ObjRef<Place, PlaceKey>>,
}

#[derive(Debug, Clone)]
pub struct ClassInstanceFiles {
    pub official: Vec<ClassInstanceFile>,
    pub community: Vec<ClassInstanceFile>,
    // pub  denied: Vec<ClassInstanceFile>,
}

#[derive(Debug, Clone)]
pub struct ClassInstanceFile {
    pub id: u32,
    pub file: File,
    pub name: String,
    pub category: FileCategory,
    pub upload_datetime: String, // FIXME use chrono
    pub uploader: Option<u32>,
    pub uploader_teacher: Option<u32>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct File {
    pub hash: String,
    pub size: u32,
    pub mime: String,
    pub license: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: StudentKey,
    pub name: String,
    pub abbreviation: Option<String>,
    pub number: u32,
    pub(crate) enrollments: Vec<ObjRef<Enrollment, EnrollmentKey>>,
    pub(crate) shifts: Vec<ObjRef<ClassShift, ShiftKey>>,
    pub first_year: Option<u32>,
    pub last_year: Option<u32>,
    pub(crate) course: Option<ObjRef<Course, CourseKey>>,
    pub avg_grade: Option<u32>,
    pub url: String,
}

#[derive(Clone)]
pub struct Teacher {
    pub id: u32,
    pub name: String,
    pub abbreviation: Option<String>,
    pub first_year: Option<u32>,
    pub last_year: Option<u32>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub thumb: Option<String>,
    pub rank: Option<String>,
    pub(crate) departments: Vec<ObjRef<Department, DepartmentKey>>,
    pub(crate) shifts: Vec<ObjRef<ClassShift, ShiftKey>>,
    pub url: String,

    pub(crate) client: Arc<Supernova>,
    pub(crate) thumb_cache: OnceCell<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Enrollment {
    pub id: EnrollmentKey,
    pub(crate) class_instance: ObjRef<ClassInstance, ClassInstanceKey>,
    pub(crate) student: ObjRef<Student, StudentKey>,
    pub attendance: Option<bool>,
    pub attendance_date: Option<String>,
    pub normal_grade: Option<u8>,
    pub normal_grade_date: Option<String>,
    pub recourse_grade: Option<u8>,
    pub recourse_grade_date: Option<String>,
    pub special_grade: Option<u8>,
    pub special_grade_date: Option<String>,
    pub improvement_grade: Option<u8>,
    pub improvement_grade_date: Option<String>,
    pub approved: Option<bool>,
    pub grade: Option<u8>,
}

impl Department {
    pub fn get_building(&self) -> Result<Option<Building>, Error> {
        Ok(if let Some(building) = &self.building {
            Some(building.coerce()?)
        } else {
            None
        })
    }

    pub fn get_courses(&self) -> Result<Vec<Course>, Error> {
        let mut result = vec![];
        for course_ref in &self.courses {
            result.push(course_ref.coerce()?);
        }
        Ok(result)
    }
}

impl PartialEq for Department {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Department {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Hash for Department {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Building {
    pub fn get_rooms(&self) -> Result<Vec<Place>, Error> {
        let mut result = vec![];
        for places_ref in &self.places {
            result.push(places_ref.coerce()?);
        }
        Ok(result)
    }
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Building {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Hash for Building {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Place {
    pub fn get_building(&self) -> Result<Option<Building>, Error> {
        Ok(if let Some(building) = &self.building {
            Some(building.coerce()?)
        } else {
            None
        })
    }
}

impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Place {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let floor_ord = self.floor.cmp(&other.floor);
        match floor_ord {
            Ordering::Equal => self.name.partial_cmp(&other.name),
            _ => Some(floor_ord),
        }
    }
}

impl Hash for Place {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Room {
    pub fn get_department(&self) -> Result<Option<Department>, Error> {
        Ok(if let Some(department) = &self.department {
            Some(department.coerce()?)
        } else {
            None
        })
    }
}

impl Course {
    pub fn get_department(&self) -> Result<Option<Department>, Error> {
        Ok(if let Some(department) = &self.department {
            Some(department.coerce()?)
        } else {
            None
        })
    }
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Course {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Hash for Course {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Class {
    pub fn get_department(&self) -> Result<Option<Department>, Error> {
        Ok(if let Some(department) = &self.department {
            Some(department.coerce()?)
        } else {
            None
        })
    }

    pub fn get_instances(&self) -> Result<Vec<ClassInstance>, Error> {
        let mut result = vec![];
        for instance_ref in &self.instances {
            result.push(instance_ref.coerce()?);
        }
        Ok(result)
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Hash for Class {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ClassInstance {
    pub fn get_department(&self) -> Result<Option<Department>, Error> {
        Ok(if let Some(department) = &self.department {
            Some(department.coerce()?)
        } else {
            None
        })
    }

    pub fn get_enrollments(&self) -> Result<Vec<Enrollment>, Error> {
        let mut result = vec![];
        for student_ref in &self.enrollments {
            result.push(student_ref.coerce()?);
        }
        Ok(result)
    }

    pub fn get_shifts(&self) -> Result<Vec<ClassShift>, Error> {
        let mut result = vec![];
        for shift_ref in &self.shifts {
            result.push(shift_ref.coerce()?);
        }
        Ok(result)
    }
}

impl PartialEq for ClassInstance {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for ClassInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.year.partial_cmp(&other.year)
    }
}

impl Hash for ClassInstance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Student {
    pub fn get_course(&self) -> Result<Option<Course>, Error> {
        Ok(if let Some(course_ref) = &self.course {
            Some(course_ref.coerce()?)
        } else {
            None
        })
    }

    pub fn get_enrollments(&self) -> Result<Vec<Enrollment>, Error> {
        let mut result = vec![];
        for enrollment_ref in &self.enrollments {
            result.push(enrollment_ref.coerce()?);
        }
        Ok(result)
    }

    pub fn get_shifts(&self) -> Result<Vec<ClassShift>, Error> {
        let mut result = vec![];
        for shift_ref in &self.shifts {
            result.push(shift_ref.coerce()?);
        }
        Ok(result)
    }
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl Hash for Student {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Teacher {
    pub fn get_departments(&self) -> Result<Vec<Department>, Error> {
        let mut result = vec![];
        for department_ref in &self.departments {
            result.push(department_ref.coerce()?);
        }
        Ok(result)
    }

    pub fn get_shifts(&self) -> Result<Vec<ClassShift>, Error> {
        let mut result = vec![];
        for shift_ref in &self.shifts {
            result.push(shift_ref.coerce()?);
        }
        Ok(result)
    }

    #[must_use]
    pub fn thumb_bytes(&self) -> Option<Result<Vec<u8>, Error>> {
        if let Some(thumb_url) = &self.thumb {
            Some(if let Some(bytes) = self.thumb_cache.get() {
                Ok(bytes.clone())
            } else {
                let response = self
                    .client
                    .base
                    .fetch_bytes(&self.client.http_client, thumb_url);
                if let Ok(bytes) = &response {
                    let _ = self.thumb_cache.set(bytes.clone());
                }
                response
            })
        } else {
            None
        }
    }
}

impl PartialEq for Teacher {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Teacher {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Hash for Teacher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Debug for Teacher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Teacher")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("abbreviation", &self.abbreviation)
            .field("first_year", &self.first_year)
            .field("last_year", &self.last_year)
            .field("phone", &self.phone)
            .field("email", &self.email)
            .field("rank", &self.rank)
            .field("url", &self.url)
            .field("thumb", &self.thumb)
            .finish()
    }
}

impl Enrollment {
    pub fn get_student(&self) -> Result<Student, Error> {
        self.student.coerce()
    }
    pub fn get_class_instance(&self) -> Result<ClassInstance, Error> {
        self.class_instance.coerce()
    }
}

impl ClassShift {
    pub fn get_teachers(&self) -> Result<Vec<Teacher>, Error> {
        let mut result = vec![];
        for teacher_ref in &self.teachers {
            result.push(teacher_ref.coerce()?);
        }
        Ok(result)
    }
}

impl PartialEq for ClassShift {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for ClassShift {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_ord = self.shift_type.cmp(&other.shift_type);
        match type_ord {
            Ordering::Equal => Some(type_ord),
            _ => self.number.partial_cmp(&other.number),
        }
    }
}

impl Hash for ClassShift {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ClassShiftInstance {
    pub fn get_place(&self) -> Result<Option<Place>, Error> {
        Ok(if let Some(room_ref) = &self.room {
            Some(room_ref.coerce()?)
        } else {
            None
        })
    }
}

impl PartialEq for ClassShiftInstance {
    fn eq(&self, other: &Self) -> bool {
        self.weekday.eq(&other.weekday)
            && self.start.eq(&other.start)
            && self.duration.eq(&other.duration)
    }
}

impl PartialOrd for ClassShiftInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let weekday_ord = self.weekday.cmp(&other.weekday);
        Some(match weekday_ord {
            Ordering::Equal => weekday_ord,
            _ => {
                let start_ord = self.start.cmp(&other.start);
                match start_ord {
                    Ordering::Equal => start_ord,
                    _ => self.duration.cmp(&other.duration),
                }
            }
        })
    }
}

impl Hash for ClassShiftInstance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.weekday.hash(state);
        self.start.hash(state);
        self.duration.hash(state);
    }
}

impl PartialEq for ClassInstanceFile {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for ClassInstanceFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Hash for ClassInstanceFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// ------------ Users ---------------

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct User {
    pub id: UserKey,
}

// ------------ Groups --------------

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GroupType {
    Institutional,
    Nuclei,
    AcademicAssociation,
    Pedagogic,
    Praxis,
    Community,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GroupVisibility {
    Secret,
    Closed,
    Request,
    Open,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GroupEventType {
    Generic,
    Talk,
    Workshop,
    Party,
    Contest,
    Fair,
    Meeting,
}

#[derive(Clone)]
pub struct Group {
    pub id: GroupKey,
    pub name: String,
    pub abbreviation: String,
    pub url: String,
    pub thumb: Option<String>,
    pub group_type: GroupType,
    pub official: bool,
    pub(crate) upgraded: Cell<bool>,

    pub(crate) client: Arc<Supernova>,
    pub(crate) outsiders_openness: OnceCell<GroupVisibility>,
    pub(crate) activities: OnceCell<Vec<GroupActivity>>,
    pub(crate) schedulings: OnceCell<Vec<GroupScheduling>>,
    pub(crate) events: OnceCell<Vec<Event>>,
    pub(crate) thumb_cache: OnceCell<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupActivity {
    Announcement(GroupAnnouncement),
    EventAnnouncement(EventAnnouncement),
    GalleryUpload(GalleryUpload),
}

#[derive(Debug, Clone)]
pub struct GroupAnnouncement {
    pub(crate) author: ObjRef<User, UserKey>,
    pub title: String,
    pub content: String,
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct EventAnnouncement {
    pub(crate) author: ObjRef<User, UserKey>,
    pub(crate) event: ObjRef<Event, EventKey>,
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GalleryUpload {
    pub(crate) author: ObjRef<User, UserKey>,
    pub datetime: DateTime<Utc>,
    pub item: GalleryItem,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd)]
pub struct GalleryItem {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd)]
pub enum GroupScheduling {
    Once(GroupSchedulingOnce),
    Periodic(GroupSchedulingPeriodic),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GroupSchedulingOnce {
    pub title: Option<String>,
    pub datetime: DateTime<Utc>,
    pub duration: u16,
    pub revoked: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GroupSchedulingPeriodic {
    pub title: Option<String>,
    pub weekday: Weekday,
    pub time: NaiveTime,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub duration: u16,
    pub revoked: bool,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: EventKey,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub duration: Option<u16>,
    pub(crate) place: Option<ObjRef<Place, PlaceKey>>,
    pub capacity: Option<u32>,
    pub cost: Option<u32>,
    pub event_type: GroupEventType,
}

impl Event {
    pub fn place(&self) -> Result<Option<Place>, Error> {
        if let Some(place) = &self.place {
            Ok(Some(place.coerce()?))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventsPage {
    pub(crate) previous_page: Option<Arc<EventsPage>>,
    pub(crate) next_page: ObjRef<Option<Arc<EventsPage>>, EventsPageKey>,
    pub(crate) items: Vec<Arc<Event>>,
}

impl Group {
    pub fn outsider_openness(&self) -> Result<GroupVisibility, Error> {
        self.upgrade()?;
        Ok(*self.outsiders_openness.get().unwrap())
    }

    pub fn activities(&self) -> Result<&[GroupActivity], Error> {
        self.upgrade()?;
        Ok(self.activities.get().unwrap())
    }

    pub fn schedulings(&self) -> Result<&[GroupScheduling], Error> {
        self.upgrade()?;
        Ok(self.schedulings.get().unwrap())
    }

    pub fn events(&self) -> Result<&[Event], Error> {
        self.upgrade()?;
        Ok(self.events.get().unwrap().as_slice())
    }

    #[must_use]
    pub fn thumb_bytes(&self) -> Option<Result<Vec<u8>, Error>> {
        if let Some(thumb_url) = &self.thumb {
            Some(if let Some(bytes) = self.thumb_cache.get() {
                Ok(bytes.clone())
            } else {
                let response = self
                    .client
                    .base
                    .fetch_bytes(&self.client.http_client, thumb_url);
                if let Ok(bytes) = &response {
                    let _ = self.thumb_cache.set(bytes.clone());
                }
                response
            })
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn upgrade(&self) -> Result<(), Error> {
        if !self.upgraded.get() {
            let group = self.client.get_group(self.id)?;

            self.outsiders_openness
                .set(*group.outsiders_openness.get().unwrap());
            self.activities.set(group.activities.get().unwrap().clone());
            self.schedulings
                .set(group.schedulings.get().unwrap().clone());
            self.events.set(group.events.get().unwrap().clone());
            self.upgraded.set(true);
        }
        Ok(())
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Group")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("abbreviation", &self.abbreviation)
            .field("url", &self.url)
            .field("thumb", &self.thumb)
            .field("type", &self.group_type)
            .field("official", &self.official)
            .field("upgraded", &self.upgraded)
            .finish()
    }
}

impl PartialOrd for GroupActivity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let datetime = match self {
            GroupActivity::Announcement(announcement) => announcement.datetime,
            GroupActivity::EventAnnouncement(event_announcement) => event_announcement.datetime,
            GroupActivity::GalleryUpload(gallery_upload) => gallery_upload.datetime,
        };
        match other {
            GroupActivity::Announcement(announcement) => {
                datetime.partial_cmp(&announcement.datetime)
            }
            GroupActivity::EventAnnouncement(event_announcement) => {
                datetime.partial_cmp(&event_announcement.datetime)
            }
            GroupActivity::GalleryUpload(gallery_upload) => {
                datetime.partial_cmp(&gallery_upload.datetime)
            }
        }
    }
}

impl GroupAnnouncement {
    pub fn author(&self) -> Result<User, Error> {
        self.author.coerce()
    }
}

impl PartialEq for GroupAnnouncement {
    fn eq(&self, other: &Self) -> bool {
        self.title.eq(&other.title)
            && self.content.eq(&other.content)
            && self.datetime.eq(&other.datetime)
    }
}

impl EventAnnouncement {
    pub fn author(&self) -> Result<User, Error> {
        self.author.coerce()
    }
    pub fn event(&self) -> Result<Event, Error> {
        self.event.coerce()
    }
}

impl PartialEq for EventAnnouncement {
    fn eq(&self, other: &Self) -> bool {
        self.event.identifier.eq(&other.event.identifier) && self.datetime.eq(&other.datetime)
    }
}

impl GalleryUpload {
    pub fn author(&self) -> Result<User, Error> {
        self.author.coerce()
    }
}

impl PartialEq for GalleryUpload {
    fn eq(&self, other: &Self) -> bool {
        self.item.eq(&other.item) && self.datetime.eq(&other.datetime)
    }
}

impl PartialOrd for GroupSchedulingOnce {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.datetime.partial_cmp(&other.datetime)
    }
}

impl PartialOrd for GroupSchedulingPeriodic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let weekday_ord = self.weekday.cmp(&other.weekday);
        Some(match weekday_ord {
            Ordering::Equal => {
                let time_ord = self.time.cmp(&other.time);
                match time_ord {
                    Ordering::Equal => self.duration.cmp(&other.duration),
                    _ => time_ord,
                }
            }
            _ => weekday_ord,
        })
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start_date.partial_cmp(&other.start_date)
    }
}

impl Hash for Event {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl EventsPage {
    #[must_use]
    pub fn items(&self) -> &[Arc<Event>] {
        self.items.as_slice()
    }

    #[must_use]
    pub fn predecessor(&self) -> Option<Arc<EventsPage>> {
        self.previous_page.clone()
    }

    pub fn successor(&self) -> Result<Option<Arc<EventsPage>>, Error> {
        self.next_page.coerce()
    }
}

// ------------ News --------------

#[derive(Debug, Clone)]
pub struct NewsPage {
    pub(crate) previous_page: Option<Arc<NewsPage>>,
    pub(crate) next_page: ObjRef<Option<Arc<NewsPage>>, NewsPageKey>,
    pub(crate) items: Vec<Arc<NewsItem>>,
}

#[derive(Clone)]
pub struct NewsItem {
    pub id: NewsItemKey,
    pub title: String,
    pub summary: String,
    pub datetime: DateTime<Utc>,
    pub thumb: Option<String>,
    pub url: String,

    pub(crate) client: Arc<Supernova>,
    pub(crate) thumb_cache: OnceCell<Vec<u8>>,
}

impl NewsPage {
    #[must_use]
    pub fn items(&self) -> &[Arc<NewsItem>] {
        self.items.as_slice()
    }

    #[must_use]
    pub fn predecessor(&self) -> Option<Arc<NewsPage>> {
        self.previous_page.clone()
    }

    pub fn successor(&self) -> Result<Option<Arc<NewsPage>>, Error> {
        self.next_page.coerce()
    }
}

impl NewsItem {
    #[must_use]
    pub fn thumb_bytes(&self) -> Option<Result<Vec<u8>, Error>> {
        if let Some(thumb_url) = &self.thumb {
            Some(if let Some(bytes) = self.thumb_cache.get() {
                Ok(bytes.clone())
            } else {
                let response = self
                    .client
                    .base
                    .fetch_bytes(&self.client.http_client, thumb_url);
                if let Ok(bytes) = &response {
                    let _ = self.thumb_cache.set(bytes.clone());
                }
                response
            })
        } else {
            None
        }
    }
}

impl PartialEq for NewsItem {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl fmt::Debug for NewsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewsItem")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("summary", &self.summary)
            .field("datetime", &self.datetime)
            .field("thumb", &self.thumb)
            .field("url", &self.url)
            .finish()
    }
}
