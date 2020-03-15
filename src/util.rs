pub fn u32_in_range(val: u32, min: u32, max: u32) -> bool {
    val >= min && val <= max
}

pub fn valid_color_temp_range(model: &str) -> (u32, u32) {
    let devices = [("LB120", (2700, 6500)), ("LB130", (2500, 9000))]
        .iter()
        .filter(|(name, _)| model.contains(name))
        .map(|(_, range)| *range)
        .collect::<Vec<_>>();
    // TODO: Verify range before returning.
    devices[0]
}
