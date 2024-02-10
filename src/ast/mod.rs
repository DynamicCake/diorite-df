use self::top::TopLevel;

pub mod recovery;
pub mod statement;
pub mod top;

pub type Span = core::ops::Range<usize>;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub data: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Self { data, span }
    }
    pub fn empty(span: Span) -> Spanned<()> {
        Spanned { data: (), span }
    }
    pub fn to_empty(self) -> Spanned<()> {
        Self::empty(self.span)
    }
    pub fn map_inner<R, F>(self, f: F) -> Spanned<R>
    where
        F: FnOnce(T) -> R,
    {
        let Self { data, span } = self;
        let res = f(data);
        Spanned::<R>::new(res, span)
    }
}

#[derive(Debug)]
pub struct MaybeSpan<T> {
    pub data: T,
    pub span: Option<Span>,
}

impl<T> MaybeSpan<T> {
    pub fn new(data: T, span: Option<Span>) -> Self {
        Self { data, span }
    }
    pub fn empty(span: Option<Span>) -> MaybeSpan<()> {
        MaybeSpan { data: (), span }
    }
    pub fn to_empty(self) -> MaybeSpan<()> {
        Self::empty(self.span)
    }
    pub fn map_inner<R, F>(self, f: F) -> MaybeSpan<R>
    where
        F: FnOnce(T) -> R,
    {
        let Self { data, span } = self;
        let res = f(data);
        MaybeSpan::<R>::new(res, span)
    }
}

pub trait CalcSpan {
    fn calculate_span(&self) -> Span;
}

pub trait TryCalcSpan {
    fn try_calculate_span(&self) -> Option<Span>;
}

#[derive(Debug)]
pub struct Program<'src> {
    top_statements: Vec<TopLevel<'src>>,
}

impl<'src> Program<'src> {
    pub fn new(top_statements: Vec<TopLevel<'src>>) -> Self {
        Self { top_statements }
    }
}

#[derive(Debug)]
pub struct Parameters<T> {
    pub items: Vec<Spanned<T>>,
}

// I do not give a shit that
impl<T> TryCalcSpan for Parameters<T>
where
    T: CalcSpan,
{
    fn try_calculate_span(&self) -> Option<Span> {
        match self.items.first() {
            Some(first) => {
                let last = self.items.last().expect("If first exists, last exists");
                Some(first.span.start..last.span.end)
            }
            None => None,
        }
    }
}

impl<T> Parameters<T> {
    pub fn new(items: Vec<Spanned<T>>) -> Self {
        Self { items }
    }
}

// StringLiteral

#[derive(Debug)]
pub struct StringLiteral<'src> {
    inner: &'src str,
}

impl<'src> StringLiteral<'src> {
    pub fn new(inner: &'src str) -> Self {
        Self { inner }
    }
}

// NumberLiteral

#[derive(Debug)]
pub struct NumberLiteral<'src> {
    inner: &'src str,
}

// Iden

#[derive(Debug)]
pub struct Iden<'src> {
    pub name: &'src str,
}

impl<'src> Iden<'src> {
    pub fn new(name: &'src str) -> Self {
        Self { name }
    }
}
