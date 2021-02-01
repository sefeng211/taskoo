pub const CREATE_TASK_TABLE_QUERY: &str = "
    create table if not exists task (
        id integer primary key,
        body text not null,
        priority integer not null,
        context_id INTEGER not null,
        created_at Text DEFAULT CURRENT_DATE,
        due_date TEXT nullable,
        scheduled_at Text nullable,
        due_repeat TEXT nullable,
        scheduled_repeat TEXT nullable,
        state_id INTEGER nullable,
        annotation TEXT nullabe,
        FOREIGN KEY(context_id) REFERENCES context(id),
        FOREIGN KEY(state_id) REFERENCES state(id)
    )";

pub const CREATE_TAG_TABLE_QUERY: &str = "
    create table if not exists tag (
        id integer primary key,
        name TEXT not null unique
    )";

pub const CREATE_TASK_TAG_TABLE_QUERY: &str = "
    create table if not exists task_tag (
        task_id integer not null,
        tag_id integer not null,
        PRIMARY KEY (task_id, tag_id),
        FOREIGN KEY (task_id) REFERENCES task(id),
        FOREIGN KEY (tag_id) REFERENCES tag(id)
    )";

pub const CREATE_DEPENDENCY_TABLE_QUERY: &str = "
    create table if not exists dependency (
        task_id integer not null,
        depended_task_id integer not null,
        PRIMARY KEY (task_id, depended_task_id),
        FOREIGN KEY (task_id) REFERENCES task(id),
        FOREIGN KEY (depended_task_id) REFERENCES task(id)
    )";

pub const CREATE_CONTEXT_TABLE_QUERY: &str = "
    create table if not exists context (
        id integer primary key,
        name Text not null unique
    )";

pub const CREATE_STATE_TABLE_QUERY: &str = "
    create table if not exists state (
        id integer primary key,
        name Text not null unique
    )";

pub fn generate_view_condition(
    context_id: &i64,
    _view_range_start: &Option<String>,
    view_range_end: &String,
    view_type: &Option<String>,
) -> Vec<String> {
    let mut conditions = generate_condition(
        &None,
        &None,
        &Some(*context_id),
        &None,
        &None,
        &None,
        &None,
        &None,
    );
    if view_type == &Some("overdue".to_string()) {
        conditions.push(
            format!("due_date < '{}'", view_range_end)
                .as_str()
                .to_string(),
        );
    } else if view_type == &Some("due".to_string()) {
        conditions.push(
            format!("(due_date <= '{}' and due_date <> '')", view_range_end)
                .as_str()
                .to_string(),
        );
        conditions.push("due_date <> ''".to_string());
    } else if view_type == &Some("schedule".to_string()) {
        conditions.push(
            format!(
                "(scheduled_at <= '{}' and scheduled_at <> '')",
                view_range_end
            )
            .as_str()
            .to_string(),
        );
    //conditions.push("scheduled_at <> ''".to_string());
    } else if view_type == &Some("all".to_string()) {
        conditions.push(
            format!("((scheduled_at <= '{}' and scheduled_at <> '') or (due_date <= '{}' and due_date <> ''))", view_range_end, view_range_end)
                .as_str()
                .to_string(),
        );
    }
    return conditions;
}

pub fn generate_get_condition(
    body: &Option<&str>,
    priority: &Option<u8>,
    context_id: &Option<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
) -> Vec<String> {
    return generate_condition(
        body,
        priority,
        context_id,
        due_date,
        scheduled_at,
        &None,
        &None,
        &None,
    );
}

pub fn generate_condition(
    body: &Option<&str>,
    priority: &Option<u8>,
    context_id: &Option<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    due_repeat: &Option<&str>,
    scheduled_repeat: &Option<&str>,
    state_id: &Option<i64>,
) -> Vec<String> {
    let mut conditions: Vec<String> = vec![];
    if body.is_some() {
        conditions.push(format!("body = '{}'", body.unwrap()).as_str().to_string());
    }

    if priority.is_some() {
        conditions.push(
            format!("priority = {}", priority.unwrap())
                .as_str()
                .to_string(),
        );
    }

    if context_id.is_some() {
        conditions.push(
            format!("context_id = {}", context_id.unwrap())
                .as_str()
                .to_string(),
        );
    }

    if due_date.is_some() {
        conditions.push(
            format!("due_date = '{}'", due_date.unwrap())
                .as_str()
                .to_string(),
        );
    }

    if scheduled_at.is_some() {
        conditions.push(
            format!("scheduled_at = '{}'", scheduled_at.unwrap())
                .as_str()
                .to_string(),
        );
    }

    if due_repeat.is_some() {
        conditions.push(
            format!("due_repeat = '{}'", due_repeat.unwrap())
                .as_str()
                .to_string(),
        );
    }

    if scheduled_repeat.is_some() {
        conditions.push(
            format!("scheduled_repeat = '{}'", scheduled_repeat.unwrap())
                .as_str()
                .to_string(),
        );
    }

    if state_id.is_some() {
        conditions.push(
            format!("state_id = {}", state_id.unwrap())
                .as_str()
                .to_string(),
        );
    }

    return conditions;
}
