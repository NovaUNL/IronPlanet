use std::fmt;
use crate::keys::*;

const UPSTREAM: &str = "https://supernova.nunl.pt/";

pub enum Endpoint {
    TokenValidation,
    Buildings,
    Building(BuildingKey),
    Rooms,
    Room(RoomKey),
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
    // ClassInstanceShifts(ClassInstanceKey),
    // ClassInstanceFiles(ClassInstanceKey),
    // ClassInstanceSchedule(ClassInstanceKey),
    // Groups,
    // Group(u32),
    // GroupMembership(u32),
    // News,
    // NewsItem(u32),
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str(UPSTREAM);
        let _ = f.write_str("api/");
        match self {
            Endpoint::Buildings => f.write_str("buildings"),
            Endpoint::Building(id) => f.write_fmt(format_args!("building/{}", id)),
            Endpoint::Room(id) => f.write_fmt(format_args!("room/{}", id)),
            Endpoint::Departments => f.write_str("departments"),
            Endpoint::Department(id) => f.write_fmt(format_args!("department/{}", id)),
            Endpoint::Courses => f.write_str("courses"),
            Endpoint::Course(id) => f.write_fmt(format_args!("course/{}", id)),
            Endpoint::Class(id) => f.write_fmt(format_args!("class/{}", id)),
            Endpoint::ClassInstance(id) => f.write_fmt(format_args!("class/i/{}", id)),
            // Endpoint::ClassInstanceShifts(id) => f.write_fmt(format_args!("class/i/{}/shifts", id)),
            // Endpoint::ClassInstanceFiles(id) => f.write_fmt(format_args!("class/i/{}/files", id)),
            // Endpoint::ClassInstanceSchedule(id) => {
            //     f.write_fmt(format_args!("class/i/{}/schedule", id))
            // }
            // Endpoint::Groups => f.write_str("groups"),
            // Endpoint::Group(id) => f.write_fmt(format_args!("group/{}", id)),
            // Endpoint::GroupMembership(id) => f.write_fmt(format_args!("group/{}/members", id)),
            // Endpoint::News => f.write_str("news"),
            // Endpoint::NewsItem(id) => f.write_fmt(format_args!("news/{}", id)),
            _ => todo!(),
        }
    }
}
