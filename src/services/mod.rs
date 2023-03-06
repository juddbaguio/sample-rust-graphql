pub mod student;

use student::StudentsCtx;

pub async fn new_student_service() -> student::StudentsCtx {
    StudentsCtx::new()
}
