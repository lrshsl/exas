#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    /// Application level code
    ///
    /// No or few eas builtins used, syntax is almost completely handled by
    /// macros rather than by the compiler | expander. Syntax on this layer
    /// could look like almost any language or a mix of languages.
    ///
    /// Typical constructs in this layer:
    /// - Macros
    ///
    /// The expansion step from HLL to CL invloves the expansion of macros.
    ///
    /// Examples:
    /// ``` eas-syn-rs
    /// let values = {90, 10, 42}.iter();
    /// let answer = loop {
    ///     if let Some(value @ 32..69) = values.next() {
    ///         break value;
    ///     }
    /// };
    /// println("The answer is {answer}");
    /// ```
    ///
    /// I guess could be possible, as well as:
    ///
    /// ``` eas-syn-rkt
    /// racket {
    ///
    /// (define (double x) (mul x x))
    ///
    /// }
    /// ```
    HighLevelLayer,

    /// C level code
    ///
    /// What persists after macro expansion. Code is on the same level as lower
    /// level languages like C.
    ///
    /// Typical constructs in this layer:
    /// - Functions
    /// - Variables
    /// - Types
    ///
    /// The expansion step from CLayer to AsL involves checking types, writing
    /// out function calls and definitions following call conventions and
    /// prototyping the storage of variables and aliases.
    ///
    /// Examples:
    /// ``` eas
    /// deg = type 3,
    ///
    /// turn = fn [what: deg] [how_much: deg] {},
    ///
    /// turn 3 90,
    /// ```
    CLayer,

    /// Hardware-independent-assembly like code
    ///
    /// The last hardware-independent layer. Code below this layer should
    /// usually not be touched by application writers.
    ///
    /// Typical constructs in this layer:
    /// - Aliases
    /// - Comptime checks (`has free GPR?`)
    AsmLayer,

    /// Hardware-dependent assembly like code
    ///
    /// Assembly level code where instructions directly correspond to
    /// instructions in the final binary. Usually CLayer functions have many
    /// different HL equivalents for different kernels, architectures and
    /// other hardware.
    ///
    /// Typical constructs in this layer:
    /// - Aliases
    HardwareLayer,

    /// The final binary
    ///
    /// Examples:
    /// ```eas-bin
    /// ELF          >          
    /// [..]
    /// ````
    ///
    /// Well you mqy want to pipe it through xxd first ;)
    /// ```xxd
    /// 00000000: 7f45 4c46 0201 0100 0000 0000 0000 0000  .ELF............
    /// 00000010: 0300 3e00 0100 0000 0097 0800 0000 0000  ..>.............
    /// [..]
    /// ```
    Binary,
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HighLevelLayer => write!(f, "hll"),
            Self::CLayer => write!(f, "cl"),
            Self::AsmLayer => write!(f, "al"),
            Self::HardwareLayer => write!(f, "hl"),
            Self::Binary => write!(f, "bin"),
        }
    }
}

impl std::str::FromStr for Layer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hll" => Ok(Self::HighLevelLayer),
            "cl" => Ok(Self::CLayer),
            "al" => Ok(Self::AsmLayer),
            "hl" => Ok(Self::HardwareLayer),
            "bin" => Ok(Self::Binary),
            _ => Err(format!("Unknown expansion layer: {}", s)),
        }
    }
}
