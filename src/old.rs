
// use iced::{
//     widget::{button, column, pick_list, row, text, text_input, Rule},
//     Alignment, Element, Length, Sandbox,
// };

// use crate::views::{
//     area::AreaView, integral_fredholm_1::Fredholm1View, integral_wolterra_2::Wolterra2View,
//     DisplayedResult, View,
// };

// #[derive(Debug, Clone)]
// pub enum Message {
//     SwitchToAreaFind,
//     SwitchToWolterra2nd,
//     SwitchToFredholm1st,
//     SetField { name: String, val: String },
//     Calculate,
//     Error(String),
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum ViewSelection {
//     AreaFind,
//     Fredholm1st,
//     Wolterra2nd,
// }

// impl ViewSelection {
//     pub fn to_name(&self) -> &'static str {
//         match self {
//             ViewSelection::AreaFind => "Find area",
//             ViewSelection::Fredholm1st => "Fredholm 1st",
//             ViewSelection::Wolterra2nd => "Wolterra 2nd",
//         }
//     }

//     pub fn from_name(name: &str) -> Self {
//         match name {
//             "Find area" => ViewSelection::AreaFind,
//             "Fredholm 1st" => ViewSelection::Fredholm1st,
//             "Wolterra 2nd" => ViewSelection::Wolterra2nd,
//             _ => unreachable!(),
//         }
//     }

//     pub fn to_message(&self) -> Message {
//         match self {
//             ViewSelection::AreaFind => Message::SwitchToAreaFind,
//             ViewSelection::Fredholm1st => Message::SwitchToFredholm1st,
//             ViewSelection::Wolterra2nd => Message::SwitchToWolterra2nd,
//         }
//     }
// }

// pub struct App {
//     cur_view: usize,
//     views: Vec<Box<dyn View>>,
//     displayed_res: Vec<DisplayedResult>,
//     errors: Vec<String>,
// }

// impl Sandbox for App {
//     type Message = Message;

//     fn new() -> Self {
//         Self {
//             cur_view: 0,
//             views: vec![
//                 Box::new(AreaView::default()),
//                 Box::new(Fredholm1View::default()),
//                 Box::new(Wolterra2View::default()),
//             ],
//             displayed_res: Vec::new(),
//             errors: Vec::new(),
//         }
//     }

//     fn title(&self) -> String {
//         "Lobanov".to_string()
//     }

//     fn update(&mut self, message: Self::Message) {
//         match message {
//             Message::SwitchToAreaFind => self.cur_view = 0,
//             Message::SwitchToFredholm1st => self.cur_view = 1,
//             Message::SwitchToWolterra2nd => self.cur_view = 2,
//             Message::SetField { name, val } => {
//                 self.views[self.cur_view].set_field(&name, val).unwrap()
//             }
//             Message::Calculate => self.displayed_res = self.views[self.cur_view].solve().unwrap(),
//             Message::Error(e) => self.errors.push(e),
//         }
//     }

//     fn view(&self) -> iced::Element<'_, Self::Message> {
//         let fields = self.views[self.cur_view].get_fields();
//         let selection = vec![
//             ViewSelection::AreaFind.to_name(),
//             ViewSelection::Fredholm1st.to_name(),
//             ViewSelection::Wolterra2nd.to_name(),
//         ];

//         row![
//             column![
//                 pick_list(
//                     selection.clone(),
//                     selection.get(self.cur_view).copied(),
//                     |name| { ViewSelection::from_name(name).to_message() },
//                 ),
//                 column(
//                     fields
//                         .into_iter()
//                         .map(|f| {
//                             row![
//                                 text(f.clone()),
//                                 text_input(
//                                     "",
//                                     self.views[self.cur_view].get_field(&f).unwrap(),
//                                     move |new_val| {
//                                         Message::SetField {
//                                             name: f.clone(),
//                                             val: new_val,
//                                         }
//                                     }
//                                 ),
//                             ]
//                         })
//                         .map(Element::from)
//                         .collect(),
//                 )
//                 .padding(20)
//                 .align_items(Alignment::Start),
//                 button(text("Calculate")).on_press(Message::Calculate)
//             ]
//             .width(Length::FillPortion(2)),
//             Rule::vertical(1),
//             column(
//                 self.displayed_res
//                     .iter()
//                     .filter_map(|r| if let DisplayedResult::Text(t) = r {
//                         Some(text(t))
//                     } else {
//                         None
//                     })
//                     .map(Element::from)
//                     .collect::<Vec<_>>()
//             )
//             .width(Length::FillPortion(3)),
//         ]
//         .padding(20)
//         .align_items(Alignment::Start)
//         .into()
//     }
// }
