use crate::network::models::{Building, BuildingPartial, Class, File, ClassInfo, ClassInfoEntry, ClassInfoSources, ClassInstance, ClassShift, ClassInstanceFile, ClassInstanceFiles, ClassShiftInstance, CoursePartial, Degree, Department, DepartmentPartial, PartialClassInstance, Period, ShiftType, Weekday, FileCategory};
use std::collections::HashMap;

#[test]
fn ok_buildings() {
    let json = r#"
[
    {
        "id": 16,
        "name": "Biblioteca",
        "abbreviation": "Bibliot.",
        "url": "/faculdade/campus/edificio/16/"
    },
    {
        "id": 19,
        "name": "Cenimat",
        "abbreviation": "Cenimat",
        "url": "/faculdade/campus/edificio/19/"
    },
    {
        "id": 17,
        "name": "Centro Excelência do Ambiente",
        "abbreviation": "CEA",
        "url": "/faculdade/campus/edificio/17/"
    }
]"#;
    let parsed: Vec<BuildingPartial> = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        vec![
            BuildingPartial {
                id: 16,
                name: "Biblioteca".to_string(),
                abbreviation: "Bibliot.".to_string()
            },
            BuildingPartial {
                id: 19,
                name: "Cenimat".to_string(),
                abbreviation: "Cenimat".to_string()
            },
            BuildingPartial {
                id: 17,
                name: "Centro Excelência do Ambiente".to_string(),
                abbreviation: "CEA".to_string()
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
        },
    )
}

#[test]
fn ok_departments() {
    let json = r#"
[
    {
        "id": 24,
        "name": "Apoio ao Ensino"
    },
    {
        "id": 18,
        "name": "Área da Física"
    },
    {
        "id": 6,
        "name": "Biologia Vegetal"
    }
]"#;

    let parsed: Vec<DepartmentPartial> = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        vec![
            DepartmentPartial {
                id: 24,
                name: "Apoio ao Ensino".to_string()
            },
            DepartmentPartial {
                id: 18,
                name: "Área da Física".to_string()
            },
            DepartmentPartial {
                id: 6,
                name: "Biologia Vegetal".to_string()
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
    "building": {
        "id": 2,
        "name": "Edifício II",
        "abbreviation": "II",
        "url": "/faculdade/campus/edificio/2/"
    },
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
        {
            "id": 29,
            "name": "Engenharia Informática",
            "abbreviation": "LEI",
            "degree": 1,
            "degree_display": "Licenciatura"
        },
        {
            "id": 186,
            "name": "Engenharia Informática",
            "abbreviation": "MEI",
            "degree": 2,
            "degree_display": "Mestrado"
        },
        {
            "id": 195,
            "name": "Engenharia Informática",
            "abbreviation": "MIEI",
            "degree": 4,
            "degree_display": "Mestrado Integrado"
        },
        {
            "id": 143,
            "name": "Engenharia Informática",
            "abbreviation": "PGEI",
            "degree": 5,
            "degree_display": "Pos-Graduação"
        }
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
            description: "Foo Bar Baz".to_string(),
            courses: vec![
                CoursePartial {
                    id: 29,
                    abbreviation: "LEI".to_string(),
                    name: "Engenharia Informática".to_string(),
                    degree: Degree::BSc
                },
                CoursePartial {
                    id: 186,
                    abbreviation: "MEI".to_string(),
                    name: "Engenharia Informática".to_string(),
                    degree: Degree::MSc
                },
                CoursePartial {
                    id: 195,
                    abbreviation: "MIEI".to_string(),
                    name: "Engenharia Informática".to_string(),
                    degree: Degree::IntegratedMSc
                },
                CoursePartial {
                    id: 143,
                    abbreviation: "PGEI".to_string(),
                    name: "Engenharia Informática".to_string(),
                    degree: Degree::PostGraduation
                }
            ],
            building: BuildingPartial {
                id: 2,
                name: "Edifício II".to_string(),
                abbreviation: "II".to_string()
            }
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
    "department": {
        "id": 12,
        "name": "Informática"
    },
    "instances": [
        {
            "id": 24767,
            "parent": 449,
            "period": 2,
            "period_display": "1º semestre",
            "year": 2018,
            "url": "/faculdade/cadeira/i/24767/"
        },
        {
            "id": 24770,
            "parent": 449,
            "period": 3,
            "period_display": "2º semestre",
            "year": 2017,
            "url": "/faculdade/cadeira/i/24770/"
        },
        {
            "id": 24766,
            "parent": 449,
            "period": 2,
            "period_display": "1º semestre",
            "year": 2020,
            "url": "/faculdade/cadeira/i/24766/"
        }
    ],
    "url": "/faculdade/cadeira/449/",
    "external_id": 11156
}"#;

    // let error = serde_json::from_str::<Class>(&json).unwrap_err();
    // println!("{}", error);
    // println!("{}", json.lines().skip(error.line()).take(1).collect::<String>());
    //
    // return;
    let parsed: Class = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed,
        Class {
            id: 449,
            name: "Atividade Prática de Desenvolvimento Curricular".to_string(),
            abbreviation: "APDC".to_string(),
            credits: 10,
            department: DepartmentPartial {
                id: 12,
                name: "Informática".to_string()
            },
            instances: vec![
                PartialClassInstance {
                    id: 24767,
                    year: 2018,
                    period: Period::FirstSemester
                },
                PartialClassInstance {
                    id: 24770,
                    year: 2017,
                    period: Period::SecondSemester
                },
                PartialClassInstance {
                    id: 24766,
                    year: 2020,
                    period: Period::FirstSemester
                },
            ],
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
    "department": {
        "id": 12,
        "name": "Informática"
    },
    "avg_grade": null,
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

    // let mut information = HashMap::new();
    // information.insert("upsteam".to_string(), class_info);

    assert_eq!(
        parsed,
        ClassInstance {
            id: 24772,
            year: 2021,
            department: DepartmentPartial {
                id: 12,
                name: "Informática".to_string()
            },
            period: Period::SecondSemester,
            students: vec![],
            teachers: vec![],
            information: ClassInfoSources {
                upstream: class_info
            },
            avg_grade: None
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
        vec![ClassShift {
            id: 42211,
            number: 1,
            shift_type: ShiftType::Seminar,
            teachers: vec![879],
            instances: vec![ClassShiftInstance {
                weekday: Weekday::Monday,
                start: 540,
                duration: 120,
                room: None
            }]
        }]
    )
}

#[test]
fn ok_class_instance_files() {
    let json = r#"
{
    "official": [
        {
            "id": "214498",
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
            official: vec![
                ClassInstanceFile {
                    id: 214498,
                    file: File {
                        hash: "3b19355d701899dc90ee77525d52ce67007bd346".to_string(),
                        size: 63488,
                        mime: "application/pdf".to_string(),
                        license: "Todos os direitos reservados".to_string(),
                        url: "/faculdade/ficheiro/3b19355d701899dc90ee77525d52ce67007bd346/".to_string()
                    },
                    name: "regulamento_estagios.pdf".to_string(),
                    category: FileCategory::Others,
                    upload_datetime: "2021-03-24T02:06:49.551799Z".to_string(),
                    uploader: None,
                    uploader_teacher: None,
                    url: "".to_string()
                }
            ],
            community: vec![]
        }
    )
}


