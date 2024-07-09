use crate::common::prelude::*;

/// A fancy [Range](`core::ops::Range<SpanSize>`) with a size of [SpanSize]
/// Don't be afraid to clone this as it is only 8 bytes
pub type Span<S = SpanSize> = core::ops::Range<S>;

/// The span size that `Span`s use
/// This shouldn't change as it is big enough for 4GB files
pub type SpanSize = u32;

/// Adds span data to (usually) a token by adding a start and end byte stored with byte indexes
#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub data: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Self { data, span }
    }
    /// Create a span for an empty struct
    /// Useful for spanning items with a known token type
    pub fn empty(span: Span) -> Spanned<()> {
        Spanned { data: (), span }
    }
    /// Create a new span with an empty value
    pub fn to_empty(&self) -> Spanned<()> {
        Self::empty(self.span.clone())
    }
    /// Maps a `Spanned<T>` to `Spanned<R>` by applying a function to the data field
    pub fn map_inner<R, F>(self, f: F) -> Spanned<R>
    where
        F: FnOnce(T) -> R,
    {
        let Self { data, span } = self;
        let res = f(data);
        Spanned::<R>::new(res, span)
    }
}

/// A [Span] but with an optional `span` field
///
/// Usually used when a list of tokens is 0.
/// A [Span] isn't used here because having 0 be the distance between start and end doesn't show
/// any highlighting to the LSP user
#[derive(Debug, PartialEq)]
pub struct MaybeSpan<T> {
    pub data: T,
    pub span: Option<Span>,
}

impl<T> MaybeSpan<T> {
    pub fn new(data: T, span: Option<Span>) -> Self {
        Self { data, span }
    }
    /// Create a span for an empty struct
    /// Useful for spanning items with a known token type
    pub fn empty(span: Option<Span>) -> MaybeSpan<()> {
        MaybeSpan { data: (), span }
    }
    /// Create a new span with an empty value
    pub fn to_empty(self) -> MaybeSpan<()> {
        Self::empty(self.span)
    }
    /// Maps a [MaybeSpan<T>] to [MaybeSpan<R>] by applying a function to the data field
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
    fn start(&self) -> SpanSize;
}

pub trait SpanEnd {
    fn end(&self) -> SpanSize;
}

pub trait TryCalcSpan {
    fn try_calculate_span(&self) -> Option<Span>;
}

pub trait TrySpanStart {
    fn try_start(&self) -> Option<SpanSize>;
}

pub trait TrySpanEnd {
    fn try_end(&self) -> Option<SpanSize>;
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

pub struct Referenced<T> {
    pub spanned: Spanned<T>,
    pub file: ProjectFile,
}

impl<T> Referenced<T> {
    pub fn new(spanned: Spanned<T>, file: ProjectFile) -> Self { Self { spanned, file } }
}

pub struct Reference {
    pub span: Span,
    pub file: ProjectFile,
}
