#[derive(Debug)]
pub enum ReferenceType {
    Command,
    Filepath,
    None,
}

#[derive(Debug)]
pub struct AutocompleteState {
    pub options: Vec<String>,
    pub current_index: usize,
    pub reference_token: Option<String>,
    pub reference_token_index: usize,
    pub reference_type: ReferenceType,
}

impl AutocompleteState {
    pub fn new() -> Self {
        Self {
            options: vec![],
            current_index: 0,
            reference_token: None,
            reference_token_index: 0,
            reference_type: ReferenceType::None,
        }
    }

    pub fn is_displayed(&self) -> bool {
        match self.reference_type {
            // only when start of the message
            ReferenceType::Command => self.reference_token_index == 0,
            ReferenceType::Filepath => true,
            _ => false,
        }
    }

    pub fn get_reference_type_from_token(token: &str) -> ReferenceType {
        match token.chars().next() {
            Some('@') => ReferenceType::Filepath,
            Some('/') => ReferenceType::Command,
            _ => ReferenceType::None,
        }
    }

    pub fn set_reference_token(
        &mut self,
        reference_token: Option<String>,
        index: usize,
        reference_type: ReferenceType,
    ) {
        self.reference_token = reference_token;
        self.reference_token_index = index;
        self.reference_type = reference_type;
    }

    pub fn get_selected_option(&self) -> String {
        if let Some(option) = self.options.get(self.current_index) {
            option.to_owned()
        } else {
            "".to_string()
        }
    }

    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = options;
    }

    pub fn set_current_index(&mut self, index: usize) {
        self.current_index = index;
    }

    pub fn increment_current_index(&mut self) {
        self.current_index = self.current_index + 1;
    }

    pub fn decrement_current_index(&mut self) {
        self.current_index = self.current_index - 1;
    }
}
