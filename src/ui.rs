use ratatui::{
    Frame, layout::{Constraint, Layout, Rect}, style::{Color, Style}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Paragraph}
};

pub fn render(frame: &mut Frame) {
    let app_layout = Layout::vertical([
        Constraint::Percentage(11), // header area (mavis title + subtitle information)
        Constraint::Percentage(2),  // spacing
        Constraint::Percentage(85), // main area (grid + sidebar)
    ]);

    let [header_area, _, main_area] = app_layout.areas(frame.area());

    draw_header(frame, header_area);
    draw_main_area(frame, main_area);
}

fn draw_header(frame: &mut Frame, header_area: Rect) {
    let subtitle_text = format!("v0.1.0 | Generating: NONE | Iterations: 0");

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

fn draw_main_area(frame: &mut Frame, main_area: Rect) {
    let main_area_layout = Layout::horizontal([
        Constraint::Percentage(70), // grid
        Constraint::Percentage(2), // spacing
        Constraint::Percentage(28), // sidebar
    ]);
    let [grid, _, sidebar_area] = main_area_layout.areas(main_area);

    draw_sidebar(frame, sidebar_area);
    draw_grid(frame, grid);
}

fn draw_sidebar(frame: &mut Frame, sidebar_area: Rect) {
    let sidebar_area_container = Layout::vertical([
        Constraint::Percentage(10), // spacing
        Constraint::Percentage(70), // sidebar list
        Constraint::Percentage(20), // sidebar description
    ]);
    let [_, sidebar, sidebar_description] = sidebar_area_container.areas(sidebar_area);

    let sidebar_description_text = Paragraph::new(
        Text::from(
            vec![
                Line::from(Span::styled("[ESC] Quit Application", Style::default().fg(Color::White)))
            ]
        )
    );
    frame.render_widget(sidebar_description_text, sidebar_description);
    frame.render_widget(
        Block::bordered().title("What would you like to do?"),
        Rect {
            x: sidebar.left(),
            y: sidebar.top(),
            width: sidebar.width,
            height: sidebar.height,
        }
    );
}

fn draw_grid(frame: &mut Frame, grid_area: Rect) {
    // -2 for internal padding
    let (grid_width, grid_height) = (grid_area.width - 2, grid_area.height - 2);

    let border_title = format!("Main Grid ({grid_width} x {grid_height})");
    let border = Block::bordered().title(border_title).border_set(border::THICK);

    frame.render_widget(border,  Rect {
        x: grid_area.left(),
        y: grid_area.top(),
        width: grid_area.width,
        height: grid_area.height,
    });
}