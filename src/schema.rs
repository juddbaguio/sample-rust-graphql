use async_graphql::{Context, EmptySubscription, MergedObject, Object, Schema};

use crate::context::{Student, StudentsCtx};

#[derive(Default)]
pub struct StudentQuery;

#[Object]
impl StudentQuery {
    async fn get_students<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Student> {
        let students_ctx = ctx.data::<StudentsCtx>().unwrap();

        students_ctx.get_students()
    }

    async fn get_student_by_id<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<Student> {
        let students_ctx = ctx.data::<StudentsCtx>().unwrap();

        students_ctx.student_by_id(id)
    }
}

#[derive(Default)]
pub struct StudentMutation;

#[Object]
impl StudentMutation {
    async fn enroll_student<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
    ) -> Result<Student, String> {
        let students_ctx = ctx.data::<StudentsCtx>().unwrap();
        if let Ok(stud) = students_ctx.add_student(name.as_str()) {
            return Ok(stud);
        };

        Err(String::from("Something wrong creating with the student"))
    }
}

#[derive(MergedObject, Default)]
pub struct RootQuery(StudentQuery);

#[derive(MergedObject, Default)]
pub struct RootMutation(StudentMutation);

pub type StudentsSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub fn init_schema(ctx: StudentsCtx) -> StudentsSchema {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription,
    )
    .data(ctx)
    .finish()
}
