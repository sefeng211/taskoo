use rusqlite::Rows;

pub struct Task {
    pub id: i64,
    pub body: String,
    pub priority: i64,
    pub context_name: String,
    pub tag_names: Vec<String>,
    pub created_at: String,
    pub due_date: String,
    pub scheduled_at: String,
    pub is_repeat: u8,
    pub is_recurrence: u8,
    pub is_completed: bool,
}

pub fn convert_rows_into_task(rows: &mut Rows) -> Vec<Task> {
    let mut tasks: Vec<Task> = vec![];

    while let Some(row) = rows.next().unwrap() {
        // TODO convert context_id to context_name
        let mut tag_names: Vec<String> = vec![];
        match row.get(14) {
            Ok(names) => {
                let temp: String = names;
                for n in temp.split(",") {
                    tag_names.push(n.to_string());
                }
            }
            Err(_) => {}
        }
        let is_completed = tag_names.contains(&"Completed".to_string());
        let task = Task {
            id: row.get(0).unwrap(),
            body: row.get(1).unwrap(),
            priority: row.get(2).unwrap(),
            tag_names: tag_names,
            created_at: row.get(4).unwrap(),
            due_date: row.get(5).unwrap(),
            scheduled_at: row.get(6).unwrap(),
            is_repeat: row.get(7).unwrap(),
            is_recurrence: row.get(8).unwrap(),
            context_name: row.get(10).unwrap(),
            is_completed: is_completed,
        };

        tasks.push(task);
    }

    tasks
}
