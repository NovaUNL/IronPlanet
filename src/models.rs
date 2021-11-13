use crate::coersion::ObjRef;

use crate::errors::Error;
use crate::keys::*;
pub use crate::network::models::{ClassInfo, ClassInfoEntry, ClassInfoSources};

#[derive(Debug, PartialEq, Clone)]
pub enum Weekday {
    Monday,
    Thursday,
    Wednesday,
    Tuesday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Period {
    Year,
    FirstSemester,
    SecondSemester,
    FirstTrimester,
    SecondTrimester,
    ThirdTrimester,
    FourthTrimester,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Degree {
    BSc,
    MSc,
    PhD,
    IntegratedMSc,
    PostGraduation,
    AdvancedStudies,
    PreGraduation,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum FileVisibility {
    Public,
    Students,
    Enrolled,
    Nobody,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Season {
    Normal,
    Exam,
    Special,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EventType {
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

#[derive(Debug, PartialEq, Clone)]
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
    pub description: String,
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
    pub(crate) students: Vec<ObjRef<Student, StudentKey>>,
    pub(crate) teachers: Vec<ObjRef<Teacher, TeacherKey>>,
    pub information: ClassInfo,
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
    pub upload_datetime: String,
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

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct Enrollment {
    pub id: EnrollmentKey,
    pub(crate) class_instance: ObjRef<ClassInstance, ClassInstanceKey>,
    pub(crate) student: ObjRef<Student, StudentKey>,
    pub attendance: bool,
    pub attendance_date: Option<String>,
    pub normal_grade: Option<u8>,
    pub normal_grade_date: Option<String>,
    pub recourse_grade: Option<u8>,
    pub recourse_grade_date: Option<String>,
    pub special_grade: Option<u8>,
    pub special_grade_date: Option<String>,
    pub improvement_grade: Option<u8>,
    pub improvement_grade_date: Option<String>,
    pub approved: bool,
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
        for course_ref in self.courses.iter() {
            result.push(course_ref.coerce()?)
        }
        Ok(result)
    }
}

impl Building {
    pub fn get_rooms(&self) -> Result<Vec<Place>, Error> {
        let mut result = vec![];
        for places_ref in self.places.iter() {
            result.push(places_ref.coerce()?)
        }
        Ok(result)
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
        for instance_ref in self.instances.iter() {
            result.push(instance_ref.coerce()?)
        }
        Ok(result)
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

    pub fn get_students(&self) -> Result<Vec<Student>, Error> {
        let mut result = vec![];
        for student_ref in self.students.iter() {
            result.push(student_ref.coerce()?)
        }
        Ok(result)
    }

    pub fn get_teachers(&self) -> Result<Vec<Teacher>, Error> {
        let mut result = vec![];
        for teacher_ref in self.teachers.iter() {
            result.push(teacher_ref.coerce()?)
        }
        Ok(result)
    }

    pub fn get_shifts(&self) -> Result<Vec<ClassShift>, Error> {
        let mut result = vec![];
        for shift_ref in self.shifts.iter() {
            result.push(shift_ref.coerce()?)
        }
        Ok(result)
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
        for enrollment_ref in self.enrollments.iter() {
            result.push(enrollment_ref.coerce()?)
        }
        Ok(result)
    }

    pub fn get_shifts(&self) -> Result<Vec<ClassShift>, Error> {
        let mut result = vec![];
        for shift_ref in self.shifts.iter() {
            result.push(shift_ref.coerce()?)
        }
        Ok(result)
    }
}

impl Teacher {
    pub fn get_departments(&self) -> Result<Vec<Department>, Error> {
        let mut result = vec![];
        for department_ref in self.departments.iter() {
            result.push(department_ref.coerce()?)
        }
        Ok(result)
    }

    pub fn get_shifts(&self) -> Result<Vec<ClassShift>, Error> {
        let mut result = vec![];
        for shift_ref in self.shifts.iter() {
            result.push(shift_ref.coerce()?)
        }
        Ok(result)
    }
}

impl Enrollment {
    pub fn get_student(&self) -> Result<Student, Error> {
        Ok(self.student.coerce()?)
    }
    pub fn get_class_instance(&self) -> Result<ClassInstance, Error> {
        Ok(self.class_instance.coerce()?)
    }
}

impl ClassShift {
    pub fn get_teachers(&self) -> Result<Vec<Teacher>, Error> {
        let mut result = vec![];
        for teacher_ref in self.teachers.iter() {
            result.push(teacher_ref.coerce()?)
        }
        Ok(result)
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
