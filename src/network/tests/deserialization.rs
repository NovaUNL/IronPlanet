use crate::network::models::{
    Building, Class, ClassInfo, ClassInfoEntry, ClassInfoSources,
    ClassInstance, ClassInstanceFile, ClassInstanceFiles, ClassShift,
    ClassShiftInstance, Department, File, FileCategory, Period, ShiftType,
    Weekday,
};

#[test]
fn ok_buildings() {
    let json = r#"
[
    {
        "id": 16,
        "name": "Biblioteca",
        "abbreviation": "Bibliot.",
        "places": [],
        "url": "/faculdade/campus/edificio/16/"
    },
    {
        "id": 19,
        "name": "Cenimat",
        "abbreviation": "Cenimat",
        "places": [],
        "url": "/faculdade/campus/edificio/19/"
    },
    {
        "id": 17,
        "name": "Centro Excelência do Ambiente",
        "abbreviation": "CEA",
        "places": [],
        "url": "/faculdade/campus/edificio/17/"
    }
]"#;
    let parsed: Vec<Building> = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        vec![
            Building {
                id: 16,
                name: "Biblioteca".to_string(),
                abbreviation: "Bibliot.".to_string(),
                places: vec![],
                cover: None,
                thumb: None
            },
            Building {
                id: 19,
                name: "Cenimat".to_string(),
                abbreviation: "Cenimat".to_string(),
                places: vec![],
                cover: None,
                thumb: None
            },
            Building {
                id: 17,
                name: "Centro Excelência do Ambiente".to_string(),
                abbreviation: "CEA".to_string(),
                places: vec![],
                cover: None,
                thumb: None
            },
        ]
    )
}

#[test]
fn ok_building() {
    let json = r#"
    {
        "id": 2,
        "name": "Edifício II",
        "abbreviation": "II",
        "places": [],
        "cover": "abc",
        "thumb": "xyz",
        "url": "/faculdade/campus/edificio/2/"
    }
"#;

    let parsed: Building = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        Building {
            id: 2,
            name: "Edifício II".to_string(),
            abbreviation: "II".to_string(),
            places: vec![],
            cover: Some("abc".to_string()),
            thumb: Some("xyz".to_string())
        },
    )
}

#[test]
fn ok_departments() {
    let json = r#"
[
    {
        "id": 24,
        "name": "Apoio ao Ensino",
        "building": 321,
        "courses": [123]
    },
    {
        "id": 18,
        "name": "Área da Física",
        "courses": []
    },
    {
        "id": 6,
        "name": "Biologia Vegetal",
        "courses": []
    }
]"#;

    let parsed: Vec<Department> = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        vec![
            Department {
                id: 24,
                name: "Apoio ao Ensino".to_string(),
                description: None,
                courses: vec![123],
                building: Some(321)
            },
            Department {
                id: 18,
                name: "Área da Física".to_string(),
                description: None,
                courses: vec![],
                building: None
            },
            Department {
                id: 6,
                name: "Biologia Vegetal".to_string(),
                description: None,
                courses: vec![],
                building: None
            },
        ]
    )
}

#[test]
fn ok_department() {
    let json = r#"
{
    "id": 12,
    "name": "Informática",
    "description": "Foo Bar Baz",
    "building":  123,
    "classes": [
        {
            "id": 1768,
            "name": "Advanced Software Development"
        },
        {
            "id": 4940,
            "name": "Álgebra Computacional"
        }
    ],
    "courses": [
    ],
    "url": "/faculdade/departamento/12/",
    "external_id": 98021
}"#;

    let parsed: Department = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        Department {
            id: 12,
            name: "Informática".to_string(),
            description: Some("Foo Bar Baz".to_string()),
            courses: vec![],
            building: Some(123)
        },
    )
}

#[test]
fn ok_class() {
    let json = r#"
{
    "id": 449,
    "name": "Atividade Prática de Desenvolvimento Curricular",
    "abbreviation": "APDC",
    "credits": 10,
    "department": 123,
    "instances": [1, 2, 3],
    "url": "/faculdade/cadeira/449/",
    "external_id": 11156
}"#;

    let parsed: Class = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        Class {
            id: 449,
            name: "Atividade Prática de Desenvolvimento Curricular".to_string(),
            abbreviation: "APDC".to_string(),
            credits: 10,
            department: Some(123),
            instances: vec![1, 2, 3],
        }
    )
}

#[test]
fn ok_class_instance() {
    let json = r#"
{
    "id": 24772,
    "parent": 449,
    "period": 3,
    "period_display": "2º semestre",
    "year": 2021,
    "information": {
        "upstream": {
            "program": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "assistance": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "extra_info": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "objectives": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "competences": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "description": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "bibliography": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "requirements": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "teaching_methods": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            },
            "evaluation_methods": {
                "en": null,
                "pt": null,
                "time": null,
                "editor": null
            }
        }
    },
    "department": 12,
    "avg_grade": null,
    "enrollments": [],
    "shifts": [],
    "url": "/faculdade/cadeira/i/24772/"
}"#;

    let parsed: ClassInstance = serde_json::from_str(&json).unwrap();

    let class_info = ClassInfo {
        program: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        assistance: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        extra_info: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        objectives: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        competences: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        description: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        bibliography: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        requirements: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        teaching_methods: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
        evaluation_methods: ClassInfoEntry {
            pt: None,
            en: None,
            time: None,
            editor: None,
        },
    };

    assert_eq!(
        parsed,
        ClassInstance {
            id: 24772,
            year: 2021,
            parent: 449,
            department: Some(12),
            period: Period::SecondSemester,
            enrollments: vec![],
            information: ClassInfoSources {
                upstream: Some(class_info)
            },
            avg_grade: None,
            shifts: vec![]
        },
    )
}

#[test]
fn ok_class_instance_shifts() {
    let json = r#"
[
    {
        "id": 42211,
        "number": 1,
        "type": 4,
        "type_display": "Seminário",
        "teachers": [
            879
        ],
        "instances": [
            {
                "weekday": 2,
                "start": 540,
                "duration": 120,
                "room": null
            }
        ],
        "url": "/faculdade/cadeira/i/24772/turno/42211"
    },
    {
        "id": 42205,
        "number": 3,
        "type": 9,
        "type_display": "Teórico-Pratico Online",
        "teachers": [],
        "instances": [],
        "url": "/faculdade/cadeira/i/24772/turno/42205"
    }
]"#;

    let parsed: Vec<ClassShift> = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        vec![
            ClassShift {
                id: 42211,
                number: 1,
                shift_type: ShiftType::Seminar,
                teachers: vec![879],
                instances: vec![ClassShiftInstance {
                    weekday: Weekday::Wednesday,
                    start: 540,
                    duration: 120,
                    room: None
                }]
            },
            ClassShift {
                id: 42205,
                number: 3,
                shift_type: ShiftType::OnlinePracticalTheoretical,
                teachers: vec![],
                instances: vec![]
            },
        ]
    )
}

#[test]
fn ok_class_instance_files() {
    let json = r#"
{
    "official": [
        {
            "id": 214498,
            "file": {
                "hash": "3b19355d701899dc90ee77525d52ce67007bd346",
                "size": 63488,
                "mime": "application/pdf",
                "license": "Todos os direitos reservados",
                "url": "/faculdade/ficheiro/3b19355d701899dc90ee77525d52ce67007bd346/"
            },
            "name": "regulamento_estagios.pdf",
            "official": true,
            "category": 9,
            "upload_datetime": "2021-03-24T02:06:49.551799Z",
            "uploader": null,
            "uploader_teacher": 879,
            "url": "/faculdade/cadeira/i/24772/ficheiro/214498"
        }
    ],
    "community": [],
    "denied": []
}"#;

    let parsed: ClassInstanceFiles = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        ClassInstanceFiles {
            official: vec![ClassInstanceFile {
                id: 214498,
                file: File {
                    hash: "3b19355d701899dc90ee77525d52ce67007bd346".to_string(),
                    size: 63488,
                    mime: "application/pdf".to_string(),
                    license: "Todos os direitos reservados".to_string(),
                    url: "/faculdade/ficheiro/3b19355d701899dc90ee77525d52ce67007bd346/"
                        .to_string()
                },
                name: "regulamento_estagios.pdf".to_string(),
                category: FileCategory::Others,
                upload_datetime: "2021-03-24T02:06:49.551799Z".to_string(),
                uploader: None,
                uploader_teacher: Some(879),
                url: "/faculdade/cadeira/i/24772/ficheiro/214498".to_string()
            }],
            community: vec![]
        }
    )
}
