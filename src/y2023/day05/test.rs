use super::*;

#[test]
fn test_translate_range_simple() {
    let mut map = Map::new();
    map.add("10 30 5");
    map.add("5 35 5");

    assert_eq!(map.translate_range(0..30), vec![(0..30)]);
    assert_eq!(map.translate_range(30..35), vec![(10..15)]);
    assert_eq!(map.translate_range(35..40), vec![(5..10)]);
    assert_eq!(map.translate_range(40..50), vec![(40..50)]);
}

#[test]
fn test_translate_range_seed_to_soil() {
    let mut map = Map::new();
    map.add("50 98 2");
    map.add("52 50 48");

    assert_eq!(map.translate_range(79..93), vec![(81..95)]);
    assert_eq!(map.translate_range(55..68), vec![(57..70)]);
}

#[test]
fn test_translate_range_soil_to_fertilizer() {
    let mut map = Map::new();
    map.add("0 15 37");
    map.add("37 52 2");
    map.add("39 0 15");

    assert_eq!(map.translate_range(57..70), vec![(57..70)]);
    assert_eq!(map.translate_range(81..95), vec![(81..95)]);
}

#[test]
fn test_translate_range_multi() {
    let mut map = Map::new();
    map.add("10 30 5");
    map.add("5 35 5");

    assert_eq!(
        map.translate_range(20..40),
        vec![(20..30), (10..15), (5..10)]
    );
    assert_eq!(
        map.translate_range(30..50),
        vec![(10..15), (5..10), (40..50)]
    );
}

#[test]
fn test_translate_range_shuffled() {
    let mut map = Map::new();
    map.add("60 5 10");
    map.add("5 35 5");
    map.add("40 20 5");
    map.add("10 30 5");

    assert_eq!(
        map.translate_range(20..45),
        vec![(40..45), (25..30), (10..15), (5..10), (40..45)]
    );
}

#[test]
fn test_merge_ranges() {
    let ranges = vec![(40..45), (25..30), (10..15), (5..10), (40..45)];
    assert_eq!(merge_ranges(ranges), vec![(5..15), (25..30), (40..45)]);
}
