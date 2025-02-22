use std::{char, io, thread, time::Duration};

use ratatui::{
    Terminal,
    crossterm::event::{self, KeyEventKind},
    prelude::CrosstermBackend,
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./day15a/src/input.txt").unwrap();

    let input_split: Vec<&str> = input.split("\r\n\r\n").map(|x| x.trim()).collect();

    let grid: Vec<Vec<char>> = input_split[0]
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let moves = input_split[1].replace("\r\n", "");
    let moves_char = moves.chars().collect::<Vec<char>>();

    let mut new_grid = wide_grid(&grid);

    let mut robot = find_robot(&new_grid);
    println!();

    let manual = false;
    if manual {
        let stdout = io::stdout();
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        terminal.clear()?;
        terminal.hide_cursor()?;

        let mut step = 0;
        let mut m = moves_char[step];

        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    // Only process the key press events
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            event::KeyCode::Char('w') => {
                                move_position(&mut new_grid, '^', &mut robot); // Move up
                            }
                            event::KeyCode::Char('s') => {
                                move_position(&mut new_grid, 'v', &mut robot); // Move down
                            }
                            event::KeyCode::Char('a') => {
                                move_position(&mut new_grid, '<', &mut robot); // Move left
                            }
                            event::KeyCode::Char('d') => {
                                move_position(&mut new_grid, '>', &mut robot); // Move right
                            }

                            event::KeyCode::Char('r') => {
                                move_position(&mut new_grid, moves_char[step], &mut robot);
                                m = moves_char[step];
                                if step < moves_char.len() {
                                    step += 1;
                                } else {
                                    break;
                                }
                            }
                            event::KeyCode::Esc => {
                                break; // Exit on Esc key
                            }
                            _ => {}
                        }
                    }
                }
            }

            // Redraw the grid after each key press (just once per event)
            terminal.draw(|f| {
                let block = Block::default().borders(Borders::ALL).title("Grid");
                f.render_widget(block, f.area());

                // Render the grid
                for (i, row) in new_grid.iter().enumerate() {
                    for (j, &cell) in row.iter().enumerate() {
                        let cell = cell.to_string();
                        let area = ratatui::layout::Rect::new(j as u16, i as u16, 1, 1);
                        let text = if cell == "@" {
                            ratatui::text::Span::from(cell).red()
                        } else {
                            ratatui::text::Span::from(cell)
                        };
                        let paragraph =
                            Paragraph::new(text).wrap(ratatui::widgets::Wrap { trim: true });
                        f.render_widget(paragraph, area);
                    }
                }
                let value = ratatui::text::Span::from(m.to_string());
                f.render_widget(
                    Paragraph::new(value).wrap(ratatui::widgets::Wrap { trim: true }),
                    f.area(),
                );
            })?;

            // Optional: Add a small sleep to prevent overloading CPU
            thread::sleep(Duration::from_millis(100));
        }

        terminal.show_cursor()?;
        println!();
    } else {
        print_grid(&new_grid);
        for m in moves.chars() {
            move_position(&mut new_grid, m, &mut robot);
        }
    }

    let mut sum = 0;
    for (row_idx, row) in new_grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '[' {
                sum += 100 * row_idx + col_idx;
            }
        }
    }
    print_grid(&new_grid);
    println!("{sum}");
    Ok(())
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &cell in row {
            print!("{cell}");
        }
        println!();
    }
}

fn find_robot(grid: &[Vec<char>]) -> Point {
    for (row_idx, row) in grid.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&c| c == '@') {
            return Point {
                x: row_idx,
                y: col_idx,
            };
        }
    }
    unreachable!("It should have a robot");
}

fn move_position(grid: &mut [Vec<char>], dir: char, position: &mut Point) {
    let mut all_boxes: Vec<Point> = Vec::new();
    let mut new_position = Point {
        x: position.x,
        y: position.y,
    };

    if dir == '^' {
        if grid[position.x - 1][position.y] == '.' {
            new_position = Point {
                x: position.x - 1,
                y: position.y,
            }
        } else if check_if_valid(grid, position, &mut all_boxes, Direction::Up) {
            all_boxes.sort_by_key(|p| p.x);
            for bosx in all_boxes {
                let temp = grid[bosx.x][bosx.y];
                grid[bosx.x][bosx.y] = '.';
                grid[bosx.x - 1][bosx.y] = temp;
            }
            new_position = Point {
                x: position.x - 1,
                y: position.y,
            }
        }
    } else if dir == 'v' {
        if grid[position.x + 1][position.y] == '.' {
            new_position = Point {
                x: position.x + 1,
                y: position.y,
            }
        } else if check_if_valid(grid, position, &mut all_boxes, Direction::Down) {
            all_boxes.sort_by_key(|p| p.x);
            for bosx in all_boxes.iter().rev() {
                let temp = grid[bosx.x][bosx.y];
                grid[bosx.x][bosx.y] = '.';
                grid[bosx.x + 1][bosx.y] = temp;
            }
            new_position = Point {
                x: position.x + 1,
                y: position.y,
            }
        }
    } else if dir == '>' {
        if grid[position.x][position.y + 1] == '.' {
            new_position = Point {
                x: position.x,
                y: position.y + 1,
            }
        } else if check_if_valid_sides(grid, position, &mut all_boxes, Direction::Right) {
            all_boxes.sort_by_key(|p| p.y);
            for bosx in all_boxes.iter().rev() {
                let temp = grid[bosx.x][bosx.y];
                grid[bosx.x][bosx.y] = '.';
                grid[bosx.x][bosx.y + 1] = temp;
            }
            new_position = Point {
                x: position.x,
                y: position.y + 1,
            }
        }
    } else if dir == '<' {
        if grid[position.x][position.y - 1] == '.' {
            new_position = Point {
                x: position.x,
                y: position.y - 1,
            }
        } else if check_if_valid_sides(grid, position, &mut all_boxes, Direction::Left) {
            all_boxes.sort_by_key(|p| p.y);
            for bosx in all_boxes {
                let temp = grid[bosx.x][bosx.y];
                grid[bosx.x][bosx.y] = '.';
                grid[bosx.x][bosx.y - 1] = temp;
            }
            new_position = Point {
                x: position.x,
                y: position.y - 1,
            }
        }
    } else {
        panic!("what are you doing");
    }

    let temp = grid[new_position.x][new_position.y];
    grid[new_position.x][new_position.y] = grid[position.x][position.y];
    grid[position.x][position.y] = temp;

    *position = new_position;

    // println!("{dir}");
    //
    // for col in grid {
    //     println!();
    //     for cell in col {
    //         print!("{cell}");
    //     }
    // }
}

fn wide_grid(old_grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_grid = Vec::new();
    for row in old_grid {
        let mut new_row = Vec::new();
        for cell in row {
            match cell {
                '#' => new_row.extend("##".chars()),
                'O' => new_row.extend("[]".chars()),
                '.' => new_row.extend("..".chars()),
                '@' => new_row.extend("@.".chars()),
                _ => {}
            }
        }
        new_grid.push(new_row);
    }
    new_grid
}

fn check_if_valid(
    new_grid: &[Vec<char>],
    curr: &Point,
    all_boxes: &mut Vec<Point>,
    direction: Direction,
) -> bool {
    let current = new_grid[curr.x][curr.y];

    // Determine the step based on the direction
    let step: isize = match direction {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => 0,
    };

    if current == '@' {
        // Recursive call in the same column, adjusting row based on direction
        return check_if_valid(
            new_grid,
            &Point {
                x: (curr.x as isize + step) as usize,
                y: curr.y,
            },
            all_boxes,
            direction,
        );
    }

    if current == '.' {
        return true;
    }
    if current == '#' {
        return false;
    }

    if current == ']' {
        let right_point = Point {
            x: curr.x,
            y: curr.y - 1,
        };

        if !all_boxes.contains(&right_point) {
            all_boxes.push(right_point);
        }

        if !all_boxes.contains(curr) {
            all_boxes.push(*curr);
        }

        // Check up-left or down-left depending on direction
        return check_if_valid(
            new_grid,
            &Point {
                x: (curr.x as isize + step) as usize,
                y: curr.y,
            },
            all_boxes,
            direction,
        ) && check_if_valid(
            new_grid,
            &Point {
                x: (curr.x as isize + step) as usize,
                y: curr.y - 1,
            },
            all_boxes,
            direction,
        );
    } else if current == '[' {
        let right_point = Point {
            x: curr.x,
            y: curr.y + 1,
        };

        if !all_boxes.contains(&right_point) {
            all_boxes.push(right_point);
        }

        if !all_boxes.contains(curr) {
            all_boxes.push(*curr);
        }
        // Check up-right or down-right depending on direction
        return check_if_valid(
            new_grid,
            &Point {
                x: (curr.x as isize + step) as usize,
                y: curr.y,
            },
            all_boxes,
            direction,
        ) && check_if_valid(
            new_grid,
            &Point {
                x: (curr.x as isize + step) as usize,
                y: curr.y + 1,
            },
            all_boxes,
            direction,
        );
    }

    false
}

fn check_if_valid_sides(
    new_grid: &[Vec<char>],
    curr: &Point,
    all_boxes: &mut Vec<Point>,
    direction: Direction,
) -> bool {
    let current = new_grid[curr.x][curr.y];

    let step: isize = match direction {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => 0,
    };

    if current == '@' {
        return check_if_valid_sides(
            new_grid,
            &Point {
                x: curr.x,
                y: (curr.y as isize + step) as usize,
            },
            all_boxes,
            direction,
        );
    };

    if current == '.' {
        return true;
    }

    if current == '#' {
        return false;
    }

    if current == ']' || current == '[' {
        all_boxes.push(Point {
            x: curr.x,
            y: curr.y,
        });
        return check_if_valid_sides(
            new_grid,
            &Point {
                x: curr.x,
                y: (curr.y as isize + step) as usize,
            },
            all_boxes,
            direction,
        );
    };

    false
}
