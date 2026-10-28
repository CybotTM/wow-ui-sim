use super::*;
use std::io::Read;

const FIXTURE: &str = include_str!("gen_items_fixture.csv");

fn generate(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let input_path = dir.path().join("ItemSparse.csv");
    std::fs::write(&input_path, input)?;
    std::fs::write(dir.path().join("ItemSearchName.csv"), "ID,Display_lang\n")?;
    let output_path = dir.path().join("output.rs");
    let mut output = File::create(&output_path)?;
    let ids = BTreeSet::from([6948, 211993]);
    build_item_map(
        &mut output,
        BufReader::new(File::open(input_path)?),
        &HashMap::new(),
        &ids,
        dir.path(),
    )?;
    drop(output);
    let mut generated = String::new();
    File::open(output_path)?.read_to_string(&mut generated)?;
    Ok(generated)
}

#[test]
fn reordered_item_sparse_columns_preserve_values() {
    let generated = generate(FIXTURE).unwrap();
    assert!(generated.contains("name: \"Hearthstone\""));
    assert!(generated.contains("name: \"Entombed Seraph's Casque\""));
    assert!(generated.contains("item_level: 571"));
    assert!(generated.contains("sell_price: 708751"));
    assert!(generated.contains("stat_percent_editor: [5259, 7889, 4685, 2315, 0, 0, 0, 0, 0, 0]"));
    assert!(
        generated.contains("stat_modifier_bonus_stat: [74, 7, 49, 32, -1, -1, -1, -1, -1, -1]")
    );
}

#[test]
fn duplicate_item_sparse_column_reports_name() {
    let input = FIXTURE.replacen("Description_lang", "Display_lang", 1);
    let error = generate(&input).unwrap_err().to_string();
    assert!(error.contains("Display_lang"), "{error}");
}

#[test]
fn truncated_item_sparse_record_reports_error() {
    let (header, _) = FIXTURE.split_once('\n').unwrap();
    let input = format!("{header}\n6948,Hearthstone\n");
    let error = generate(&input).unwrap_err().to_string();
    assert!(error.contains("ItemSparse"), "{error}");
}

#[test]
fn missing_required_item_sparse_column_reports_name() {
    let input = FIXTURE.replacen("StatPercentEditor_0", "UnknownColumn", 1);
    let error = generate(&input).unwrap_err().to_string();
    assert!(error.contains("StatPercentEditor_0"), "{error}");
}
