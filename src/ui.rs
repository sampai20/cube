use super::app::{App, State};

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Paragraph, Text, Widget};
use tui::{backend::Backend, Frame};

// TODO: Seperate this draw method into a set of private methods to separate concerns
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(size);

    // TITLE

    let title = layout[0];

    Paragraph::new([Text::styled("CubeTimer", Style::default().fg(Color::Blue))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, title);

    // CURRENT SOLVE INFO

    let info = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(layout[1]);

    let time_string = match app.state {
        State::Idle | State::Timing => match app.active_timer {
            Some(timer) => format!("{:.4}", timer.time().as_secs_f64()),
            None => {
                if app.times.is_empty() {
                    "0.0000".to_string()
                } else {
                    format!("{:.4}", app.times.last().unwrap().as_secs_f64())
                }
            }
        },
        State::Inspection(time) => time.to_string(),
    };

    Paragraph::new(
        [Text::styled(
            &app.scramble.to_string(),
            Style::default().fg(Color::Red),
        )]
        .iter(),
    )
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .wrap(true)
    .render(f, info[0]);

    Paragraph::new([Text::styled(&time_string, Style::default().fg(Color::Red))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, info[1]);

    Paragraph::new(
        [Text::styled(
            &"3x3".to_string(),
            Style::default().fg(Color::Red),
        )]
        .iter(),
    )
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .wrap(true)
    .render(f, info[2]);

    // STATS SECTION

    let stats = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ]
            .as_ref(),
        )
        .split(layout[2]);

    let recent = stats[0];
    let averages = stats[1];
    let graph = stats[2];

    // TODO: Limit the number of times you pull, this would get wasteful the moment they go off
    // the screen
    let text: Vec<Text> = app
        .times
        .iter()
        .rev()
        .map(|s| {
            Text::styled(
                format!("{:.4}", s.as_secs_f64()) + "\n",
                Style::default().fg(Color::White),
            )
        })
        .collect();

    Paragraph::new(text.iter())
        .block(
            Block::default()
                .title("Recent Solves")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Left)
        .render(f, recent);

    let mut worst = 0.0;

    let text: Vec<Text> = app
        .compute_statistics()
        .iter()
        .map(|(label, value)| {
            let value = match value {
                Some(v) => {
                    // TODO: This is whack
                    if label == "worst" {
                        worst = v.as_secs_f64().ceil();
                    }
                    format!("{:.4}", v.as_secs_f64())
                }
                None => String::from("NA"),
            };
            let text = format!("{}: {}\n", label, value);
            Text::styled(text, Style::default().fg(Color::White))
        })
        .collect();

    Paragraph::new(text.iter())
        .block(Block::default().title("Average").borders(Borders::ALL))
        .alignment(Alignment::Left)
        .render(f, averages);

    // TODO: Move this logic elsewhere
    let n = match app.times.len() & 1 {
        0 => app.times.len(),
        1 => app.times.len() + 1,
        _ => unreachable! {},
    };

    let points: Vec<(f64, f64)> = app
        .times
        .iter()
        .enumerate()
        .map(|(i, time)| (i as f64, time.as_secs_f64()))
        .collect();

    let dataset = Dataset::default()
        .name("All time solves")
        .marker(Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&points);

    Chart::default()
        .block(Block::default().title("Your Solves").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Solve")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, n as f64])
                .labels(&["0.0", &n.to_string()]),
        )
        .y_axis(
            Axis::default()
                .title("Time")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, worst * 1.5]) // TODO(magic-number)
                .labels(&["0.0", &worst.to_string()]),
        )
        .datasets(&[dataset])
        .render(f, graph);
}
