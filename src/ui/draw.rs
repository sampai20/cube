use super::*;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Paragraph, Text, Widget};
use tui::{backend::Backend, Frame};

pub fn draw_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
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

    draw_title(f, layout[0]);
    draw_solve_info(f, layout[1], builder::SolveInfo::from(app));
    draw_stats(f, layout[2], builder::Stats::from(app));
}

fn draw_title<B: Backend>(f: &mut Frame<B>, section: Rect) {
    Paragraph::new([Text::styled("CubeTimer", Style::default().fg(Color::Blue))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, section);
}

fn draw_solve_info<B: Backend>(f: &mut Frame<B>, section: Rect, data: builder::SolveInfo) {
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
        .split(section);

    Paragraph::new([Text::styled(data.scramble, Style::default().fg(Color::Red))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, info[0]);

    Paragraph::new([Text::styled(data.time, Style::default().fg(Color::Red))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, info[1]);

    Paragraph::new(
        [Text::styled(
            data.cube_type,
            Style::default().fg(Color::Red),
        )]
        .iter(),
    )
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .wrap(true)
    .render(f, info[2]);
}

fn draw_stats<B: Backend>(f: &mut Frame<B>, section: Rect, data: builder::Stats) {
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
        .split(section);

    let recent = stats[0];
    let averages = stats[1];
    let graph = stats[2];

    Paragraph::new(
        [Text::styled(
            data.recent_solves,
            Style::default().fg(Color::White),
        )]
        .iter(),
    )
    .block(
        Block::default()
            .title("Recent Solves")
            .borders(Borders::ALL),
    )
    .alignment(Alignment::Left)
    .render(f, recent);

    Paragraph::new([Text::styled(data.stats, Style::default().fg(Color::White))].iter())
        .block(Block::default().title("Average").borders(Borders::ALL))
        .alignment(Alignment::Left)
        .render(f, averages);

    let dataset = Dataset::default()
        .name("All time solves")
        .marker(Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&data.graph.points);

    let x_bounds = data.graph.x_axis.bounds;
    let x_bounds = [x_bounds.0, x_bounds.1];

    let y_bounds = data.graph.y_axis.bounds;
    let y_bounds = [y_bounds.0, y_bounds.1];

    Chart::default()
        .block(Block::default().title("Your Solves").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Solve")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds(x_bounds)
                .labels(&data.graph.x_axis.labels),
        )
        .y_axis(
            Axis::default()
                .title("Time")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds(y_bounds)
                .labels(&data.graph.y_axis.labels),
        )
        .datasets(&[dataset])
        .render(f, graph);
}
