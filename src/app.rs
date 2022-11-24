use crate::views::{
    area::AreaView, integral_fredholm_1::Fredholm1View, integral_wolterra_2::Wolterra2View,
    DisplayedResult, Error, View,
};

#[derive(Debug, Clone)]
pub enum Message {
    SwitchToView(usize),
    EnterInField { name: String, val: String },
    FieldError { name: String, err: String },
    RuntimeError { err: String },
    Calculate,
}

pub struct AppState {
    cur_view_index: usize,
    views: Vec<Box<dyn View>>,
    runtime_errors: Vec<String>,
    field_errors: Vec<(String, String)>,
    displayed_result: Vec<DisplayedResult>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cur_view_index: 0,
            views: vec![
                Box::new(AreaView::default()),
                Box::new(Fredholm1View::default()),
                Box::new(Wolterra2View::default()),
            ],
            runtime_errors: Vec::new(),
            field_errors: Vec::new(),
            displayed_result: Vec::new(),
        }
    }

    fn cur_view(&self) -> &dyn View {
        self.views[self.cur_view_index].as_ref()
    }

    fn cur_view_mut(&mut self) -> &mut dyn View {
        self.views[self.cur_view_index].as_mut()
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SwitchToView(i) => {
                self.cur_view_index = i;
                self.runtime_errors.clear();
                self.field_errors.clear();
            }
            Message::EnterInField { name, val } => {
                self.cur_view_mut().set_field(&name, val).map_or_else(
                    |e| self.field_errors.push((name.clone(), format!("{:?}", e))),
                    |()| (),
                )
            }
            Message::FieldError { name, err } => self.field_errors.push((name, err)),
            Message::RuntimeError { err } => self.runtime_errors.push(err),
            Message::Calculate => {
                self.runtime_errors.clear();
                self.field_errors.clear();

                self.displayed_result = self.cur_view().solve().map_or_else(
                    |e| {
                        match e {
                            Error::InvalidField { name, err } => {
                                self.field_errors.push((name, err))
                            }
                            Error::Runtime(e) => self.runtime_errors.push(e),
                        };
                        vec![]
                    },
                    |res| res,
                );
            }
        }
    }

    pub fn get_cur_view_fields(&self) -> Vec<String> {
        self.cur_view().get_fields()
    }

    pub fn get_cur_view_name(&self) -> String {
        match self.cur_view_index {
            0 => "Area".to_string(),
            1 => "Fredholm 1st Kind".to_string(),
            2 => "Wolterra 2nd Kind".to_string(),
            _ => unreachable!(),
        }
    }

    pub fn get_view_names(&self) -> Vec<String> {
        vec![
            "Area".to_string(),
            "Fredholm 1st Kind".to_string(),
            "Wolterra 2nd Kind".to_string(),
        ]
    }

    pub fn view_name_to_message(&self, name: &str) -> Message {
        match name {
            "Area" => Message::SwitchToView(0),
            "Fredholm 1st Kind" => Message::SwitchToView(1),
            "Wolterra 2nd Kind" => Message::SwitchToView(2),
            _ => unreachable!(),
        }
    }

    pub fn get_result(&self) -> &[DisplayedResult] {
        &self.displayed_result
    }

    pub fn get_runtime_errors(&self) -> &[String] {
        &self.runtime_errors
    }

    pub fn get_field_errors(&self) -> &[(String, String)] {
        &self.field_errors
    }

    pub fn get_field_val(&self, field: &str) -> Option<&str> {
        self.cur_view().get_field(field)
    }
}
