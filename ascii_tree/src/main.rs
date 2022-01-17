const NUMBER_OF_SEGMENTS: u32 = 6;
const ROWS_IN_FIRST_SEGMENT: u32 = 5;

const WIDTH_FIRST: f32 = (ROWS_IN_FIRST_SEGMENT - 1) as f32;
const WIDTH_LAST: f32 = WIDTH_FIRST + (NUMBER_OF_SEGMENTS - 2) as f32;
const WIDTH_SUM: f32 = ((WIDTH_FIRST + WIDTH_LAST) / 2.0) * (NUMBER_OF_SEGMENTS - 1) as f32;
const TOTAL_TREE_WIDTH: u32 = 1 + 2 * (ROWS_IN_FIRST_SEGMENT - 1) + 2 * (WIDTH_SUM) as u32;

const HEIGHT_FIRST: f32 = (ROWS_IN_FIRST_SEGMENT) as f32;
const HEIGHT_LAST: f32 = 2.0 * HEIGHT_FIRST + (NUMBER_OF_SEGMENTS - 1) as f32;
const TOTAL_TREE_HEIGHT: u32 = ((HEIGHT_FIRST + HEIGHT_LAST) / 2.0) as u32 * NUMBER_OF_SEGMENTS;

const LOG_WIDTH: u32 = 1 + 2 * (ROWS_IN_FIRST_SEGMENT - 1);
const LOG_HEIGHT: u32 = NUMBER_OF_SEGMENTS;

fn main() {
    print!("\x1b[?3l");
    clear_terminal();
    hide_cursor();

    let timer = std::time::Instant::now();
    loop {
        print_tree();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        if timer.elapsed() > std::time::Duration::from_millis(10000) {
            break;
        }
    }

    clear_terminal();
    show_cursor();
}

fn print_tree() {
    let mut length = 1;
    let mut rows = ROWS_IN_FIRST_SEGMENT;
    let mut cursor_position = move_cursor_to_position(((TOTAL_TREE_WIDTH - 1) / 2 + 1, 1));

    for _ in 0..NUMBER_OF_SEGMENTS {
        for _ in 0..rows {
            print!("{}", get_row(length));
            length += 2;
            cursor_position = move_cursor_to_position((cursor_position.0 - 1, cursor_position.1 + 1));
        }
        rows += 1;
        length -= 4;
        cursor_position = move_cursor_to_position((cursor_position.0 + 2, cursor_position.1));
    }

    cursor_position = move_cursor_to_position((((TOTAL_TREE_WIDTH - 1) / 2 + 1) - (LOG_WIDTH - 1) / 2, cursor_position.1));
    for _ in 0..LOG_HEIGHT {
        print!("{}", wrap_rgb(&"#".repeat(LOG_WIDTH as usize), (0x9B, 0x67, 0x3C)));
        cursor_position = move_cursor_to_position((cursor_position.0, cursor_position.1 + 1));
    }
}

fn get_row(length: usize) -> String {
    let mut row = "*".repeat(length);

    let mut bubble= rand::random::<usize>();
    if bubble % 4 == 0 {
        bubble = bubble % length;
        row.replace_range(bubble..bubble + 1, "@");
    }

    color_row(&mut row);
    row
}

fn color_row(row: &mut String) {
    let bubble_position = row.find("@");
    const GREEN: (u8, u8, u8) = (0x00, 0xFF, 0x00);
    const RED: (u8, u8, u8) = (0xFF, 0x00, 0x00);
    const OFFSET: usize = "\x1b[0m".len() + 1;

    match bubble_position {
        None => {
            *row = wrap_rgb(&row, GREEN);
        }
        Some(position) => {
            row.replace_range(..position, &*wrap_rgb(&row[..position], GREEN));
            let position = row.find("@").unwrap();
            row.replace_range(position..=position, &*wrap_rgb(&row[position..=position], RED));
            let position = row.find("@").unwrap();
            row.replace_range(position+OFFSET.., &*wrap_rgb(&row[position+OFFSET..], GREEN));
        }
    }
}

fn wrap_rgb(text: &str, color: (u8, u8, u8)) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", color.0, color.1, color.2, text)
}

fn move_cursor_to_position(position: (u32, u32)) -> (u32, u32) {
    print!("\x1b[{};{}H", position.1, position.0);
    position
}

fn clear_terminal() {
    print!("\x1b[1;1H\x1b[0J");
}

fn hide_cursor() {
    print!("\x1b[?25l");
}

fn show_cursor() {
    print!("\x1b[?25h");
}
