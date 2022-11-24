use app::{AppState, Message};
use common::function::Function;
use iced::{
    widget::{
        button, canvas,
        canvas::{Cache, Path, Program, Stroke},
        column, pick_list, row, text, text_input, Rule,
    },
    Alignment, Color, Element, Length, Point, Sandbox, Settings, Theme,
};
use views::DisplayedResult;

extern crate iced;

mod app;
mod area_calc;
mod common;
mod integral_eq;
mod mathparse;
mod min_find;
mod spline;
mod views;

struct Graph {
    pts: Vec<Vec<(f64, f64)>>,
    cached_paths: Cache,
    cached_grid: Cache,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl Graph {
    fn new(funcs: &[(Box<dyn Function<Error = String>>, f64, f64)]) -> Result<Self, String> {
        let n = 100;
        let pts = funcs
            .iter()
            .map(|(f, from, to)| f.sample(*from, *to, n))
            .collect::<Result<Vec<_>, _>>()?;

        let min_x = pts
            .iter()
            .map(|func| func.iter().map(|(x, _)| *x).reduce(f64::min).unwrap())
            .reduce(f64::min)
            .unwrap();

        let max_x = pts
            .iter()
            .map(|func| func.iter().map(|(x, _)| *x).reduce(f64::max).unwrap())
            .reduce(f64::max)
            .unwrap();

        let min_y = pts
            .iter()
            .map(|func| func.iter().map(|(_, y)| *y).reduce(f64::min).unwrap())
            .reduce(f64::min)
            .unwrap();

        let max_y = pts
            .iter()
            .map(|func| func.iter().map(|(_, y)| *y).reduce(f64::max).unwrap())
            .reduce(f64::max)
            .unwrap();

        Ok(Self {
            cached_paths: Cache::default(),
            cached_grid: Cache::default(),
            pts,
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

impl<Message> Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let transform = |x: f64, y: f64| {
            (
                ((x - self.min_x) / (self.max_x - self.min_x)) as f32 * bounds.width,
                (bounds.height
                    - ((y - self.min_y) / (self.max_y - self.min_y)) as f32 * bounds.height),
            )
        };

        let graphs = self.cached_paths.draw(bounds.size(), |frame| {
            self.pts.iter().for_each(|func_pts| {
                let path = Path::new(|path| {
                    for (x, y) in func_pts {
                        let (x, y) = transform(*x, *y);
                        path.line_to(Point::new(x, y));
                    }
                });

                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_color(Color::from_rgb(1.0, 0.0, 0.0))
                        .with_width(2.0),
                );
            });
        });

        let grid = self.cached_grid.draw(bounds.size(), |frame| {
            for i in (self.min_x.floor() as i32)..=(self.max_x.ceil() as i32) {
                let path = Path::new(|path| {
                    let (x0, y0) = transform(i as f64, self.min_y);
                    let (x1, y1) = transform(i as f64, self.max_y);

                    path.line_to(Point::new(x0, y0));
                    path.line_to(Point::new(x1, y1));
                });

                frame.stroke(
                    &path,
                    Stroke::default().with_color(Color::BLACK).with_width(1.0),
                );
            }

            for i in (self.min_y.floor() as i32)..=(self.max_y.ceil() as i32) {
                let path = Path::new(|path| {
                    let (x0, y0) = transform(self.min_x, i as f64);
                    let (x1, y1) = transform(self.max_x, i as f64);

                    path.line_to(Point::new(x0, y0));
                    path.line_to(Point::new(x1, y1));
                });

                frame.stroke(
                    &path,
                    Stroke::default().with_color(Color::BLACK).with_width(1.0),
                );
            }
        });

        vec![graphs, grid]
    }
}

pub struct App {
    state: AppState,
}

impl Sandbox for App {
    type Message = Message;
    fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    fn title(&self) -> String {
        "Lobanov".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        self.state.update(message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let field_errors = self
            .state
            .get_field_errors()
            .iter()
            .map(|(f, e)| {
                text(format!("{f} - {e}"))
                    .style(Color::from_rgb(1.0, 0.0, 0.0))
                    .size(10)
            })
            .map(Element::from)
            .collect::<Vec<_>>();

        let mut field_list = self
            .state
            .get_cur_view_fields()
            .into_iter()
            .map(|n| {
                (
                    text(n.clone()).width(Length::FillPortion(3)).size(15),
                    text_input(
                        "",
                        self.state.get_field_val(&n).unwrap_or(""),
                        move |new_val| Message::EnterInField {
                            name: n.to_owned(),
                            val: new_val,
                        },
                    )
                    .width(Length::FillPortion(5))
                    .size(15),
                )
            })
            .map(|(label, field)| row![label, field])
            .map(Element::from)
            .collect::<Vec<_>>();

        let mut info_column = vec![];
        info_column.push(
            pick_list(
                self.state.get_view_names(),
                Some(self.state.get_cur_view_name()),
                |n| self.state.view_name_to_message(&n),
            )
            .into(),
        );
        info_column.append(&mut field_list);
        info_column.push(button("Calculate").on_press(Message::Calculate).into());
        info_column.push(row(field_errors).into());

        let mut res_column = self
            .state
            .get_result()
            .iter()
            .filter_map(|res| match res {
                DisplayedResult::Text(t) => Some(Element::from(text(t).size(15))),
                DisplayedResult::Functions(funcs) => Some(Element::from({
                    let g = Graph::new(funcs).unwrap();
                    canvas(g)
                        .width(Length::Units(300))
                        .height(Length::Units(300))
                })),
                DisplayedResult::FunctionNDim {
                    f: _,
                    from: _,
                    to: _,
                } => None,
                DisplayedResult::TextFile {
                    path: _,
                    contents: _,
                } => None,
            })
            .collect::<Vec<_>>();

        let mut runtime_errors = self
            .state
            .get_runtime_errors()
            .iter()
            .map(|e| {
                text(format!("Runtime Error - {e}"))
                    .style(Color::from_rgb(1.0, 0.0, 0.0))
                    .size(10)
            })
            .map(Element::from)
            .collect::<Vec<_>>();

        res_column.append(&mut runtime_errors);

        let elems = vec![
            column(info_column).width(Length::FillPortion(2)).into(),
            Rule::vertical(1).into(),
            column(res_column).width(Length::FillPortion(5)).into(),
        ];

        row(elems)
            .padding(5)
            .spacing(10)
            .align_items(Alignment::Start)
            .into()
    }
}

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (640, 480);
    App::run(settings).expect("Error: ")
}
