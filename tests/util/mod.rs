pub enum GoldenFileNames {
    Main { base_name: String },
    Generated { base_name: String },
}

impl GoldenFileNames {
    pub fn new(base_name: &str, override_main: bool) -> Self {
        if override_main {
            Self::Main {
                base_name: base_name.to_string(),
            }
        } else {
            Self::Generated {
                base_name: base_name.to_string(),
            }
        }
    }

    pub fn target(&self) -> String {
        match self {
            Self::Main { base_name } => format!("{}_main.png", base_name),
            Self::Generated { base_name } => format!("{}_gen.png", base_name),
        }
    }

    pub fn main(&self) -> String {
        match self {
            Self::Main { base_name } | Self::Generated { base_name } => {
                format!("{}_main.png", base_name)
            }
        }
    }

    pub fn diff(&self) -> String {
        match self {
            Self::Main { base_name } | Self::Generated { base_name } => {
                format!("{}_diff.png", base_name)
            }
        }
    }

    pub fn is_override(&self) -> bool {
        matches!(self, Self::Main { .. })
    }
}
