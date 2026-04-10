use ratatui::{
    Frame, layout::{Constraint, Layout, Rect}, style::{Color, Style}, symbols::border, text::{Line, Span, Text}, widgets::{Block, List, Paragraph},
    prelude::Stylize
};

use crate::{app, grid, sidebar};

pub fn render(frame: &mut Frame, app: &mut app::App) {
    let app_layout = Layout::vertical([
        Constraint::Percentage(11), // header area (mavis title + subtitle information)
        Constraint::Percentage(2),  // spacing
        Constraint::Percentage(85), // main area (grid + sidebar)
    ]);

    let [header_area, _, main_area] = app_layout.areas(frame.area());

    draw_header(frame, header_area, &app.grid);
    draw_main_area(frame, main_area, app);
}

fn draw_header(frame: &mut Frame, header_area: Rect, grid: &grid::Grid) {
    let algorithm_name = if let Some(algorithm) = &grid.algorithm { algorithm.name } else { "NONE" };
    let iterations = if let Some(algorithm) = &grid.algorithm { algorithm.current_index } else { 0 };
    let subtitle_text = format!("v0.1.0 | Generating: {algorithm_name} | Iterations: {iterations}");

    let header_lines = vec![
        Line::from(Span::styled("                  __ ", Style::default().fg(Color::White))),
        Line::from(Span::styled("|\\/|  /\\  \\  / | /__`", Style::default().fg(Color::White))),
        Line::from(Span::styled("|  | /~~\\  \\/  | .__/", Style::default().fg(Color::White))),
        Line::from("\n"),
        Line::from(subtitle_text)
    ];

    // todo: is there a better way to make a paragraph with multiple lines?
    frame.render_widget(Paragraph::new(Text::from(header_lines)), header_area);
}

fn draw_main_area(frame: &mut Frame, main_area: Rect, app: &mut app::App) {
    let main_area_layout = Layout::horizontal([
        Constraint::Percentage(70), // grid
        Constraint::Percentage(2), // spacing
        Constraint::Percentage(28), // sidebar
    ]);
    let [grid_area, _, sidebar_area] = main_area_layout.areas(main_area);

    draw_sidebar(frame, sidebar_area, &mut app.sidebar);
    draw_grid(frame, grid_area, &mut app.grid);
}

fn draw_sidebar(frame: &mut Frame, sidebar_area: Rect, sidebar: &mut sidebar::Sidebar) {
    let sidebar_area_container = Layout::vertical([
        Constraint::Percentage(10), // spacing
        Constraint::Percentage(70), // sidebar list
        Constraint::Percentage(20), // sidebar description
    ]);
    let [_, sidebar_container, sidebar_description] = sidebar_area_container.areas(sidebar_area);

    let sidebar_description_text = Paragraph::new(
        Text::from(
            vec![
                Line::from(Span::styled("[↑/↓] Scroll Up/Down", Style::default().fg(Color::White))),
                Line::from(Span::styled("[ENTER] Select Option", Style::default().fg(Color::White))),
                Line::from(Span::styled("[ESC] Quit Application", Style::default().fg(Color::White)))
            ]
        )
    );
    frame.render_widget(sidebar_description_text, sidebar_description);

    let options = List::new(
        sidebar.page
            .options()
            .iter()
            .map(|o| o.title)
    )
        .block(Block::bordered().title("What would you like to do?"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(" >> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(
        options,
        Rect {
            x: sidebar_container.left(),
            y: sidebar_container.top(),
            width: sidebar_container.width,
            height: sidebar_container.height,
        },
        &mut sidebar.state
    );
}

fn draw_grid(frame: &mut Frame, grid_area: Rect, grid: &mut grid::Grid) {
    // -2 for internal padding
    let (grid_width, grid_height) = (grid_area.width - 2, grid_area.height - 2);
    
    // regenerate grid if there are inconsistencies between console grid and grid state
    if grid.width() != grid_width || grid.height() != grid_height {
        grid.reset(Some((grid_width, grid_height)));
        grid.bounds.0 = (grid_area.left() + 1, grid_area.top() + 1);
        grid.bounds.1 = (grid_area.right() - 2, grid_area.bottom() - 2);
    }

    // Draw grid box
    let border_title = format!("Main Grid ({grid_width} x {grid_height})");
    let border = Block::bordered().title(border_title).border_set(border::THICK);

    frame.render_widget(border,  Rect {
        x: grid_area.left(),
        y: grid_area.top(),
        width: grid_area.width,
        height: grid_area.height,
    });

    // Draw nodes on screen
    let grid_content: Vec<Line> = grid.nodes.iter().map(|row| {
        let nodes: Vec<Span> = row.iter().map(|node| node.span()).collect();
        Line::from(nodes)
    }).collect();

    frame.render_widget(Paragraph::new(Text::from(grid_content)), Rect {
        x: grid_area.left() + 1,
        y: grid_area.top() + 1,
        width: grid_width,
        height: grid_height,
    });
}