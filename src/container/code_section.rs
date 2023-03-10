/// EOF Code Section
/// 
/// The code section contains a vector of functions with the minimum length being one.
/// While a function is required for the creation of the code section, it may be a single empty
/// function to accomodate data contracts.
pub struct CodeSection<'a> {
    /// Function Vector
    /// 
    /// Function at index zero MUST be the entry point of the program.
    functions: Vec<Function<'a>>,
}

impl<'a> CodeSection<'a> {
    /// Create new Code Section. At least one function is required as the entry point.
    pub fn new(entry_point: Function<'a>, functions: Option<Vec<Function<'a>>>) -> Self {
        let functions = match functions {
            Some(mut funs) => {
                funs.insert(0, entry_point);
                funs
            },
            None => vec![entry_point],
        };

        Self { functions }
    }

    /// Get the "entry point" function
    pub fn entry_point(&self) -> &Function {
        &self.functions.get(0).expect("entry point not found (this should never happen)")
    }

    /// Get all functions
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Get number of functions
    pub fn number_of_functions(&self) -> u16 {
        self.functions.len() as u16
    }

    /// Get the total size fo the code section
    pub fn size(&self) -> u16 {
        self.functions.iter().map(|fun| fun.len() as u16).sum()
    }
}

/// Code Section Function
pub struct Function<'a> {
    /// Function instructions
    instructions: &'a [u8],
    /// Input stack height
    inputs: u8,
    /// Output stack height
    outputs: u8,
    /// Max stack height within the function
    max_stack_height: u16,
}

impl<'a> Function<'a> {
    /// Get the instructions directly
    pub fn instructions(&self) -> &[u8] {
        self.instructions
    }

    /// Get the length of the instructions
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Get the expected stack input
    pub fn inputs(&self) -> u8 {
        self.inputs
    }

    /// Get the expected stack output
    pub fn outputs(&self) -> u8 {
        self.outputs
    }

    /// Get the max stack height
    pub fn max_stack_height(&self) -> u16 {
        self.max_stack_height
    }
}
