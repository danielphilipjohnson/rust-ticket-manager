use crate::db::schema::issues;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::issues)]
pub struct Issue {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub assigned_to: Option<String>,
    pub status: String,
    pub is_open: bool,
}

#[derive(Insertable)]
#[diesel(table_name = issues)]
pub struct NewIssue<'a> {
    pub project_id: i32,
    pub title: &'a str,
    pub description: &'a str,
    pub created_by: &'a str,
    pub assigned_to: Option<&'a str>,
    pub status: &'a str,
    pub is_open: bool,
}

impl Issue {
    pub fn create(
        conn: &mut PgConnection,
        project_id: i32,
        title: &str,
        description: &str,
        created_by: &str,
        assigned_to: Option<&str>,
        status: &str,
    ) -> QueryResult<Issue> {
        let new_issue = NewIssue {
            project_id,
            title,
            description,
            created_by,
            assigned_to,
            status,
            is_open: true,
        };

        diesel::insert_into(issues::table)
            .values(&new_issue)
            .returning(Issue::as_returning())
            .get_result(conn)
    }

    pub fn get_by_project(conn: &mut PgConnection, proj_id: i32) -> QueryResult<Vec<Issue>> {
        issues::table
            .filter(issues::project_id.eq(proj_id))
            .select(Issue::as_select())
            .load(conn)
    }

    pub fn get_by_id(conn: &mut PgConnection, issue_id: i32) -> QueryResult<Issue> {
        issues::table
            .find(issue_id)
            .select(Issue::as_select())
            .first(conn)
    }

    pub fn update(&self, conn: &mut PgConnection, changes: IssueChanges) -> QueryResult<Issue> {
        diesel::update(issues::table.find(self.id))
            .set(changes)
            .returning(Issue::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, issue_id: i32) -> QueryResult<bool> {
        let count = diesel::delete(issues::table.find(issue_id)).execute(conn)?;

        Ok(count > 0)
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = issues)]
pub struct IssueChanges {
    pub title: Option<String>,
    pub description: Option<String>,
    pub assigned_to: Option<Option<String>>,
    pub status: Option<String>,
    pub is_open: Option<bool>,
}
