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

pub trait SpanStart {
    fn start(&self) -> usize;
}

pub trait SpanEnd {
    fn end(&self) -> usize;
}

pub trait TryCalcSpan {
    fn try_calculate_span(&self) -> Option<Span>;
}

pub trait TrySpanStart {
    fn try_start(&self) -> Option<usize>;
}

pub trait TrySpanEnd {
    fn try_end(&self) -> Option<usize>;
}

pub trait CalcThenWrap
where
    Self: Sized,
{
    fn calculate_span_wrap(self) -> Spanned<Self>;
}

impl<T> CalcThenWrap for T
where
    T: CalcSpan,
    T: Sized,
{
    fn calculate_span_wrap(self) -> Spanned<Self> {
        let span = self.calculate_span();
        Spanned::new(self, span)
    }
}

pub trait TryCalcThenWrap
where
    Self: Sized,
{
    fn try_calculate_span_wrap(self) -> MaybeSpan<Self>;
}

impl<T> TryCalcThenWrap for T
where
    T: TryCalcSpan,
    T: Sized,
{
    fn try_calculate_span_wrap(self) -> MaybeSpan<Self> {
        let span = self.try_calculate_span();
        MaybeSpan::new(self, span)
    }
}
