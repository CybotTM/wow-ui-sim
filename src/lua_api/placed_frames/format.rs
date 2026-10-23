//! The external layout-local.txt grammar; unknown keys do not delimit records.
use super::{POINTS, Placement};
use crate::widget::AnchorPoint;
use std::fmt::Write;

pub(super) fn write_records(records: &[Placement]) -> String {
    let mut text = String::from("Version: 1\n");
    for record in records {
        writeln!(
            text,
            "Frame: {}\nFrameLevel: {}\nAnchor: {}\nX: {}\nY: {}\nW: {}\nH: {}",
            record.name,
            record.level,
            record.point.as_str(),
            record.x,
            record.y,
            record.width,
            record.height,
        )
        .expect("writing to a String cannot fail");
    }
    text
}

pub(super) fn read_records(text: &str) -> Vec<Placement> {
    let mut records = Vec::new();
    let mut current = None;
    for line in text.split(['\r', '\n']) {
        let Some((key, value)) = line.split_once(": ") else {
            continue;
        };
        if key.eq_ignore_ascii_case("Frame") {
            if let Some(record) = current.take() {
                records.push(record);
            }
            current = Some(empty_record(value));
        } else if let Some(record) = current.as_mut() {
            read_field(record, key, value);
        }
    }
    if let Some(record) = current {
        records.push(record);
    }
    records
}

fn empty_record(name: &str) -> Placement {
    Placement {
        name: name.into(),
        level: -1,
        point: AnchorPoint::TopLeft,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }
}

fn read_field(record: &mut Placement, key: &str, value: &str) {
    match key.to_ascii_lowercase().as_str() {
        "framelevel" => record.level = read_integer_prefix(value),
        "x" => record.x = read_integer_prefix(value),
        "y" => record.y = read_integer_prefix(value),
        "w" => record.width = read_integer_prefix(value),
        "h" => record.height = read_integer_prefix(value),
        "anchor" => {
            if let Some(point) = POINTS
                .iter()
                .find(|point| point.as_str().eq_ignore_ascii_case(value))
            {
                record.point = *point;
            }
        }
        _ => {}
    }
}

fn read_integer_prefix(value: &str) -> i32 {
    let length = value
        .bytes()
        .enumerate()
        .take_while(|(index, byte)| byte.is_ascii_digit() || (*index == 0 && *byte == b'-'))
        .count();
    value[..length].parse().unwrap_or(0)
}
