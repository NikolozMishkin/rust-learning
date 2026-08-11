use std::time::Duration;
use tokio::time::sleep;

struct Project {
    id: u32,
    name: String,
    duration_days: u32,
}

impl Project {
    fn new(id: u32, name: &str, duration_days: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            duration_days,
        }
    }
}

#[tokio::main]
async fn main() {
    let (id1, id2) = (1, 2);
    let project1 = read_details_from_db(id1).await.unwrap();
    let project2 = read_details_from_db(id2).await.unwrap();
    if project1.duration_days > project2.duration_days {
        println!(
            "{} takes {} days more than {}",
            project1.name,
            project1.duration_days - project2.duration_days,
            project2.name
        );
    } else if project2.duration_days > project1.duration_days {
        println!(
            "{} takes {} days more than {}",
            project2.name,
            project2.duration_days - project1.duration_days,
            project1.name
        );
    } else {
        println!(
            "Both {} and {} take the same number of days",
            project1.name, project2.name
        );
    }
}

async fn read_details_from_db(id: u32) -> Result<Project, String> {
    // dummy read from database
    sleep(Duration::from_millis(1000)).await;
    let database = [
        Project::new(1, "Project Alpha", 30),
        Project::new(2, "Project Beta", 45),
        Project::new(3, "Project Gamma", 30),
    ];
    for project in database {
        if id == project.id {
            return Ok(project);
        }
    }
    Err("Project record not present".into())
}
