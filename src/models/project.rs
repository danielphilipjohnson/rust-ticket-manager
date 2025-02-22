use crate::db::schema::projects;
use diesel::prelude::*;
use diesel::AsChangeset;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::db::schema::projects)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Project {
    pub fn create(
        conn: &mut PgConnection,
        project_name: &str,
        project_description: &str,
    ) -> Result<Project, diesel::result::Error> {
        use crate::db::schema::projects::dsl::*;

        let new_project = NewProject {
            name: project_name,
            description: project_description,
        };

        diesel::insert_into(projects)
            .values(&new_project)
            .returning(Project::as_returning())
            .get_result(conn)
    }
    pub fn get_first_n(
        conn: &mut PgConnection,
        limit: i64,
    ) -> Result<Vec<Project>, diesel::result::Error> {
        use crate::db::schema::projects::dsl::*;

        projects
            .limit(limit)
            .select(Project::as_select())
            .load::<Project>(conn)
    }

    pub fn delete_by_id(conn: &mut PgConnection, project_id: i32) -> QueryResult<usize> {
        use crate::db::schema::projects::dsl::*;

        diesel::delete(projects)
            .filter(id.eq(project_id))
            .execute(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, project_id: i32) -> QueryResult<Option<Project>> {
        use crate::db::schema::projects::dsl::*;

        projects
            .filter(id.eq(project_id))
            .select((id, name, description))
            .first(conn)
            .optional()
    }

    pub fn update(
        conn: &mut PgConnection,
        project_id: i32,
        updates: &UpdateProject,
    ) -> QueryResult<Project> {
        use crate::db::schema::projects::dsl::*;

        diesel::update(projects.filter(id.eq(project_id)))
            .set(updates)
            .returning(Project::as_returning())
            .get_result(conn)
    }
}
