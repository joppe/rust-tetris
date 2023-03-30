use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}, Result
};

struct Postion {
    x: u16,
    y: u16,
}

fn render_block<W: Write>(buffer: &mut W, position: Postion) {
    buffer.queue(cursor::MoveTo(position.x, position.y)).unwrap();
    buffer.queue(style::PrintStyledContent("█".magenta())).unwrap();
}

type TetrominoRow = [u8; 4];
type TetrominoGrid = [TetrominoRow; 4];

fn main() -> Result<()> {
    let mut stdout = stdout();
    let tetromino: TetrominoGrid = [
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for (row, columns) in tetromino.iter().enumerate() {
        for (column, &value) in columns.iter().enumerate() {
            if value == 1 {
                let position_1 = Postion{x: 2 * column as u16, y: row as u16};
                let position_2 = Postion{x: position_1.x + 1, y: position_1.y};

                render_block(&mut stdout, position_1);
                render_block(&mut stdout, position_2);
            }
        }
    }
    
    stdout.flush()?;

    Ok(())
}
