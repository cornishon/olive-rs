use olive::canvas::Canvas;
use std::io;

const BG_COLOR: u32 = 0xFF181818;
const FG_COLOR: u32 = 0xFF1818DD;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn checker_example(canvas: &mut Canvas) -> io::Result<()> {
    let (cols, rows) = (8, 6);
    let (cell_w, cell_h) = (WIDTH / cols, HEIGHT / rows);
    canvas.fill(BG_COLOR);

    for y in 0..rows {
        for x in 0..cols {
            canvas.fill_rect(
                if (x + y) % 2 == 0 { BG_COLOR } else { FG_COLOR },
                (x * cell_w) as i64,
                (y * cell_h) as i64,
                cell_w,
                cell_h,
            );
        }
    }

    canvas.fill_rect(0xFF1818DD, -100, -100, 50, 50);
    canvas.save_to_ppm("checker.ppm")
}

fn circle_example(canvas: &mut Canvas) -> io::Result<()> {
    let radius = 100;
    canvas.fill(BG_COLOR);
    canvas.fill_circle(FG_COLOR, (WIDTH / 2) as i64, (HEIGHT / 2) as i64, radius);
    canvas.fill_circle(0xFF18FF18, WIDTH as i64 * 2, HEIGHT as i64 * 2, radius);
    canvas.save_to_ppm("circle.ppm")
}

fn lines_example(canvas: &mut Canvas) -> io::Result<()> {
    canvas.fill(BG_COLOR);
    let w = WIDTH as i64;
    let h = HEIGHT as i64;
    canvas.line(FG_COLOR, 0, 0, w, h);
    canvas.line(FG_COLOR, w, 0, 0, h);
    canvas.line(0xFF18FF18, 0, 0, w / 4, h);
    canvas.line(0xFF18FF18, w / 4, 0, 0, h);
    canvas.line(0xFF18FF18, w, 0, w / 4 * 3, h);
    canvas.line(0xFF18FF18, w / 4 * 3, 0, w, h);
    canvas.line(0xFFFF3030, 0, h / 2, w, h / 2);
    canvas.line(0xFFFF3030, w / 2, 0, w / 2, h);

    canvas.save_to_ppm("lines.ppm")
}

fn main() -> io::Result<()> {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    checker_example(&mut canvas)?;
    circle_example(&mut canvas)?;
    lines_example(&mut canvas)?;

    Ok(())
}
