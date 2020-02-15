use super::Token;
pub use logos::source::WithSource;
pub use logos::{Lexer, Logos, Source};
use std::fmt;
pub use std::ops::Range;
use std::ops::{Deref, DerefMut};

pub struct LexerWrapper<Source>(pub Lexer<Token, Source>);

impl<'source, Source> Deref for LexerWrapper<Source>
where
    Token: self::Logos + WithSource<Source> + Copy + PartialEq,
    Source: self::Source<'source> + Copy,
{
    type Target = Lexer<Token, Source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source, Source> DerefMut for LexerWrapper<Source>
where
    Token: self::Logos + WithSource<Source> + Copy + PartialEq,
    Source: self::Source<'source> + Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Adding the ability to use with iterators.
/// This also allow for iterator features like `Peek`
#[derive(Clone)]
pub struct TokenItem<Token: Logos, Source> {
    /// Current token in the iteration.
    pub token: Token,
    source: Source,

    token_start: usize,
    token_end: usize,
}

impl<'source, Token, Source> fmt::Debug for TokenItem<Token, Source>
where
    Token: self::Logos + WithSource<Source> + fmt::Debug,
    Source: self::Source<'source>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({:?})", self.token, self.slice())
    }
}

impl<'source, Token, Source> TokenItem<Token, Source>
where
    Token: self::Logos + WithSource<Source>,
    Source: self::Source<'source>,
{
    /// Get the range for the current token in `Source`.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.token_start..self.token_end
    }

    /// Get a string slice of the current token.
    #[inline]
    pub fn slice(&self) -> Source::Slice {
        unsafe {
            self.source
                .slice_unchecked(self.token_start..self.token_end)
        }
    }
}

impl<'source, Source> Iterator for LexerWrapper<Source>
where
    Source: self::Source<'source> + Copy,
{
    type Item = TokenItem<Token, Source>;

    fn next(&mut self) -> Option<Self::Item> {
        if Token::LineComment == self.token {
            self.advance();
        }

        if Token::END == self.token {
            None
        } else {
            let range = self.range();
            let res = Some(TokenItem {
                token: self.token,
                source: self.source,
                token_start: range.start,
                token_end: range.end,
            });
            self.advance();
            res
        }
    }
}
