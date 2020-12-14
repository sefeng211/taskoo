pub fn generate_view_condition(
    context_id: &i64,
    _view_range_start: &Option<String>,
    view_range_end: &String,
    view_type: &Option<String>,
) -> Vec<String> {
    let mut conditions: Vec<String> = vec![];
    if view_type == &Some("overdue".to_string()) {
        conditions =
            generate_condition(&None, &None, &Some(*context_id), &None, &None, &None, &None);
        conditions.push(
            format!("due_date < '{}'", view_range_end)
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
    is_repeat: &Option<u8>,
    is_recurrence: &Option<u8>,
) -> Vec<String> {
    return generate_condition(
        body,
        priority,
        context_id,
        due_date,
        scheduled_at,
        is_repeat,
        is_recurrence,
    );
}

pub fn generate_condition(
    body: &Option<&str>,
    priority: &Option<u8>,
    context_id: &Option<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    is_repeat: &Option<u8>,
    is_recurrence: &Option<u8>,
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

    if is_repeat.is_some() {
        conditions.push(
            format!("is_repeat = {}", is_repeat.unwrap())
                .as_str()
                .to_string(),
        );
    }

    if is_recurrence.is_some() {
        conditions.push(
            format!("is_recurrence = {}", is_recurrence.unwrap())
                .as_str()
                .to_string(),
        );
    }

    return conditions;
}
