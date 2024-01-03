use std::collections::HashMap;
use crate::frontend::location::SourceRange;
use crate::types::Type;

#[derive(Clone)]
pub struct Symbol {
    /// name associated with this symbol
    pub(crate) name: String,
    /// this symbol's type
    pub(crate) tp: Type,
    /// where this symbol is located in source
    pub(crate) loc: SourceRange,
}

type ScopeFrame = HashMap<String, Symbol>;

#[derive(Clone)]
pub struct SymbolTable {
    /// stack of symbol frames. Bottom most is global scope, and then they lower etc.
    frames: Vec<ScopeFrame>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let global_frame = ScopeFrame::new();
        Self {
            frames: Vec::from([global_frame])
        }
    }

    /// push a new scope frame to the stack and return the newly returned frame's mutable reference
    pub fn push_scope(&mut self) {
        self.frames.push(ScopeFrame::new());
    }

    /// pops the top of the frame stack and returns it
    pub fn pop_scope(&mut self) -> Option<ScopeFrame> {
        self.frames.pop()
    }

    /// checks if the given symbol name is defined anywhere. Returns the symbol if it does, or
    /// None if it doesn't exist
    pub fn symbol_defined<StrT: AsRef<str>>(&self, name: StrT) -> Option<&Symbol> {
        let name = name.as_ref();
        for frame in self.frames.iter().rev() {
            if frame.contains_key(name) {
                return frame.get(name);
            }
        }

        None
    }

    /// checks if the given symbol is defined in the current scope
    pub fn symbol_defined_in_current_scope<StrT: AsRef<str>>(&self, name: StrT) -> Option<&Symbol> {
        self.frames.last()
            .and_then(|frame| frame.get(name.as_ref()))
    }
    
    /// retrieves the type of the symbol if it exists, or unknown if it is not defined
    pub fn symbol_type_or_unknown<StrT: AsRef<str>>(&self, name: StrT) -> Type {
        self.symbol_defined(name.as_ref())
            .map(|symbol| symbol.tp)
            .unwrap_or(Type::Unknown)
    }

    /// add a new symbol to the current scope
    pub fn add_symbol(&mut self, symbol: Symbol) {
        let current_frame = self.frames.last_mut().unwrap();
        current_frame.insert(symbol.name.clone(), symbol);
    }
}
