use mysql::*;
use mysql::prelude::*;
use chrono::NaiveTime;
use crate::model::schedule::Schedule;

pub struct ScheduleDAO {
    pool: Pool,
}

impl ScheduleDAO {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub fn init_schedule_table(&self) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS schedules (
                id INT AUTO_INCREMENT PRIMARY KEY,
                course_name VARCHAR(255) NOT NULL,
                teacher VARCHAR(255) NOT NULL,
                classroom VARCHAR(255) NOT NULL,
                day_of_week INT NOT NULL,
                start_time TIME NOT NULL,
                end_time TIME NOT NULL
            )")?;
        Ok(())
    }

    pub async fn create_schedule(&self, schedule: Schedule) -> Result<Schedule> {
        let mut conn = self.pool.get_conn()?;
        let schedule_id = conn.exec_drop(
            r"INSERT INTO schedules (course_name, teacher, classroom, day_of_week, start_time, end_time)
              VALUES (:course_name, :teacher, :classroom, :day_of_week, :start_time, :end_time)",
            params! {
                "course_name" => &schedule.course_name,
                "teacher" => &schedule.teacher,
                "classroom" => &schedule.classroom,
                "day_of_week" => schedule.day_of_week,
                "start_time" => &schedule.start_time,
                "end_time" => &schedule.end_time,
            }
        ).map(|_| {
            conn.last_insert_id() as i32
        })?;

        Ok(Schedule {
            id: Some(schedule_id),
            ..schedule
        })
    }

    pub async fn get_all_schedules(&self) -> Result<Vec<Schedule>> {
        let mut conn = self.pool.get_conn()?;
        let schedules = conn.query_map(
            "SELECT id, course_name, teacher, classroom, day_of_week, start_time, end_time FROM schedules",
            |(id, course_name, teacher, classroom, day_of_week, start_time, end_time)| {
                Schedule {
                    id: Some(id),
                    course_name,
                    teacher,
                    classroom,
                    day_of_week,
                    start_time,
                    end_time,
                }
            }
        )?;
        Ok(schedules)
    }

    pub async fn get_schedule_by_id(&self, id: i32) -> Result<Schedule> {
        let mut conn = self.pool.get_conn()?;
        let schedule = conn.exec_map(
            "SELECT id, course_name, teacher, classroom, day_of_week, start_time, end_time FROM schedules WHERE id = :id",
            params! {
                "id" => id
            },
            |(id, course_name, teacher, classroom, day_of_week, start_time, end_time): (i32, String, String, String, i32, mysql::Value, mysql::Value)| {
                Schedule {
                    id: Some(id),
                    course_name,
                    teacher,
                    classroom,
                    day_of_week,
                    start_time: NaiveTime::parse_from_str(&start_time.as_sql(false).replace("'", ""), "%H:%M:%S%.f").unwrap().to_string(),
                    end_time: NaiveTime::parse_from_str(&end_time.as_sql(false).replace("'", ""), "%H:%M:%S%.f").unwrap().to_string(),
                }
            }
        )?;
        
        match schedule.into_iter().next() {
            Some(s) => Ok(s),
            None => Err(Error::from(DriverError::MissingNamedParameter("id".to_string()))),
        }
    }

    pub async fn update_schedule(&self, id: i32, schedule: Schedule) -> Result<Schedule> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            r"UPDATE schedules SET course_name = :course_name, teacher = :teacher, 
              classroom = :classroom, day_of_week = :day_of_week, start_time = :start_time, 
              end_time = :end_time WHERE id = :id",
            params! {
                "id" => id,
                "course_name" => &schedule.course_name,
                "teacher" => &schedule.teacher,
                "classroom" => &schedule.classroom,
                "day_of_week" => schedule.day_of_week,
                "start_time" => &schedule.start_time,
                "end_time" => &schedule.end_time,
            }
        )?;
        
        Ok(Schedule {
            id: Some(id),
            ..schedule
        })
    }

    pub async fn delete_schedule(&self, id: i32) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "DELETE FROM schedules WHERE id = :id",
            params! {
                "id" => id
            }
        )?;
        Ok(())
    }
}