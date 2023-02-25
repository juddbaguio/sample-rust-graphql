use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::schema::StudentObj;

#[derive(Clone)]
pub struct Student {
    pub id: String,
    pub full_name: String,
}
#[derive(Default)]
pub struct StudentsCtx {
    pub students: Arc<Mutex<HashMap<String, Student>>>,
}

impl StudentsCtx {
    pub fn new() -> Self {
        let mut ctx = Self::default();
        ctx.seed();

        ctx
    }

    fn seed(&mut self) {
        let name_list = ["Judd Misael R. Baguio", "Keith Yvonne C. Saycon"];
        let mut db: HashMap<String, Student> = HashMap::new();
        for name in name_list {
            let key = Uuid::new_v4().to_string();
            db.insert(
                key.clone(),
                Student {
                    id: key,
                    full_name: name.to_string(),
                },
            );
        }

        self.students = Arc::new(Mutex::new(db))
    }

    pub fn get_students(&self) -> Vec<StudentObj> {
        let ref_db = Arc::clone(&self.students);
        let mut student_list: Vec<StudentObj> = vec![];
        for student in ref_db.lock().unwrap().values() {
            student_list.push(StudentObj(Student {
                id: student.id.clone(),
                full_name: student.full_name.clone(),
            }))
        }

        student_list
    }

    pub fn student_by_id(&self, id: String) -> Option<StudentObj> {
        let students_ref = Arc::clone(&self.students);
        if let Some(student) = students_ref.lock().unwrap().get(&id) {
            return Some(StudentObj(Student {
                id: student.id.clone(),
                full_name: student.full_name.clone(),
            }));
        }

        None
    }

    pub fn add_student(&self, name: &str) -> Result<StudentObj, ()> {
        let students_ref = Arc::clone(&self.students);
        let key = Uuid::new_v4().to_string();
        students_ref.lock().unwrap().insert(
            key.clone(),
            Student {
                id: key.clone(),
                full_name: name.clone().to_string(),
            },
        );

        Ok(StudentObj(Student {
            id: key,
            full_name: name.to_string(),
        }))
    }
}
