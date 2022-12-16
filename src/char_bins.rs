pub fn remap_char_to_flatten_loc(c: char) -> usize {
    return match c {
        'a'..='z' => c as u32 - 'a' as u32,
        'A'..='Z' => c as u32 - 'A' as u32 + 26,
        '0'..='9' => c as u32 - '0' as u32,
        _ => 0,
    } as usize;
}
