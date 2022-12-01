use iced::{
    theme,
    widget::{
        button, canvas,
        canvas::{Cache, Path, Program, Stroke},
        column, pick_list, row, scrollable, text, text_input, Rule,
    },
    Color, Element, Length, Point, Sandbox, Settings, Theme,
};
use prac_2022_11::{
    app::{AppState, ProblemName},
    problems::{
        graph::{Graph, PathKind, Viewport},
        SolutionParagraph,
    },
};

extern crate iced;

struct App {
    state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetField { name: String, val: String },
    ClearSolution { index: usize },
    Solve,
    None,
    SelectProblem(ProblemName),
}

impl Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        _: &Self::State,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::widget::canvas::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        let bounds_viewport = Viewport::new(0.0, bounds.width as f64, bounds.height as f64, 0.0);

        let funcs = Cache::default().draw(bounds.size(), |frame| {
            for p in &self.paths {
                let path = Path::new(|path| {
                    for (x, y) in &p.pts {
                        let (x, y) = Viewport::convert(&self.viewport, &bounds_viewport, (*x, *y));
                        path.line_to(Point::new(x as f32, y as f32));
                    }
                });

                match p.kind {
                    PathKind::Line => frame.stroke(
                        &path,
                        Stroke::default()
                            .with_color(Color::from_rgb(p.color.0, p.color.1, p.color.2))
                            .with_width(2.0),
                    ),
                    PathKind::Filled => {
                        frame.fill(&path, Color::from_rgb(p.color.0, p.color.1, p.color.2))
                    }
                }
            }
        });

        let grid = Cache::default().draw(bounds.size(), |frame| {
            for i in (self.viewport.left.floor() as i32)..=(self.viewport.right.ceil() as i32) {
                let path = Path::new(|path| {
                    let (x0, y0) = Viewport::convert(
                        &self.viewport,
                        &bounds_viewport,
                        (i as f64, self.viewport.top),
                    );
                    let (x1, y1) = Viewport::convert(
                        &self.viewport,
                        &bounds_viewport,
                        (i as f64, self.viewport.bottom),
                    );

                    path.line_to(Point::new(x0 as f32, y0 as f32));
                    path.line_to(Point::new(x1 as f32, y1 as f32));
                });

                frame.stroke(
                    &path,
                    Stroke::default().with_color(Color::BLACK).with_width(1.0),
                );
            }

            for i in (self.viewport.bottom.floor() as i32)..=(self.viewport.top.ceil() as i32) {
                let path = Path::new(|path| {
                    let (x0, y0) = Viewport::convert(
                        &self.viewport,
                        &bounds_viewport,
                        (self.viewport.left, i as f64),
                    );
                    let (x1, y1) = Viewport::convert(
                        &self.viewport,
                        &bounds_viewport,
                        (self.viewport.right, i as f64),
                    );

                    path.line_to(Point::new(x0 as f32, y0 as f32));
                    path.line_to(Point::new(x1 as f32, y1 as f32));
                });

                frame.stroke(
                    &path,
                    Stroke::default().with_color(Color::BLACK).with_width(1.0),
                );
            }

            frame.fill_text(format!(
                "x from {:.2} to {:.2}, y from {:.2} to {:.2}",
                self.viewport.left, self.viewport.right, self.viewport.bottom, self.viewport.top
            ));
        });

        vec![funcs, grid]
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            state: AppState::default(),
        }
    }

    fn title(&self) -> String {
        "Lobanov".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SetField { name, val } => {
                self.state.set_field(&name, val);
                self.state.validate();
            }
            Message::Solve => {
                self.state.validate();
                self.state.solve();
            }
            Message::None => {}
            Message::ClearSolution { index } => self.state.rem_solution(index),
            Message::SelectProblem(p) => self.state.set_problem(p),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut left_column_elems = vec![];
        left_column_elems.push(
            pick_list(
                self.state.get_problems(),
                self.state.get_cur_problem(),
                Message::SelectProblem,
            )
            .into(),
        );

        let mut form = self
            .state
            .fields()
            .map(|(name, val)| {
                (
                    text(name),
                    text_input("", val, |new_val| Message::SetField {
                        name: name.to_string(),
                        val: new_val,
                    }),
                )
            })
            .map(|(t, f)| row![t, f])
            .map(Element::from)
            .collect();

        let mut validation_errors = self
            .state
            .get_validation_errors()
            .iter()
            .map(|e| text(e.0.clone()).style(Color::from_rgb(1.0, 0.0, 0.0)))
            .map(Element::from)
            .collect();

        left_column_elems.append(&mut form);
        left_column_elems.push(
            button("Solve")
                .on_press(if self.state.get_validation_errors().is_empty() {
                    Message::Solve
                } else {
                    Message::None
                })
                .style(if self.state.get_validation_errors().is_empty() {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                })
                .into(),
        );
        left_column_elems.append(&mut validation_errors);

        let left_column = Element::from(scrollable(
            column(left_column_elems).width(Length::FillPortion(2)),
        ));

        let solutions = self
            .state
            .get_solutions()
            .map(|s| {
                s.explanation
                    .iter()
                    .map(|e| match e {
                        SolutionParagraph::Text(t) => Element::from(text(t)),
                        SolutionParagraph::Graph(g) => Element::from(
                            canvas(g)
                                .width(Length::Units(300))
                                .height(Length::Units(300)),
                        ),
                        SolutionParagraph::RuntimeError(e) => {
                            Element::from(text(e).style(Color::from_rgb(1.0, 0.0, 0.0)))
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .enumerate()
            .map(|(i, mut s)| {
                s.push(Element::from(
                    button("x")
                        .style(theme::Button::Destructive)
                        .on_press(Message::ClearSolution { index: i }),
                ));
                s.push(Rule::horizontal(1).into());
                s
            })
            .fold(vec![], |mut acc, mut c| {
                acc.append(&mut c);
                acc
            });

        let right_column = Element::from(scrollable(
            column(solutions).width(Length::FillPortion(5)).padding(10),
        ));

        row![left_column, right_column].into()
    }
}

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (640, 480);
    App::run(settings).expect("Error: ")
}
