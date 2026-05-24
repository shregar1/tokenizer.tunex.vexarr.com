//! Borrowed views over tokenized batches.

use super::TokenizedBatch;

/// Borrowed tokenized batch view.
#[derive(Clone, Copy, Debug)]
pub struct TokenizedBatchView<'a> {
    /// Backing batch.
    pub(crate) batch: &'a TokenizedBatch,
}

impl<'a> TokenizedBatchView<'a> {
    /// Returns one sample view.
    #[must_use]
    pub fn sample(self, index: usize) -> Option<TokenizedSampleView<'a>> {
        let offset = usize::try_from(*self.batch.offsets.get(index)?).ok()?;
        let length = usize::try_from(*self.batch.lengths.get(index)?).ok()?;
        let end = offset.checked_add(length)?;

        Some(TokenizedSampleView {
            input_ids: self.batch.input_ids.get(offset..end)?,
            attention_mask: self.batch.attention_mask.get(offset..end)?,
            id: self.batch.sample_ids.get(index)?.as_deref(),
        })
    }
}

/// Borrowed view of one tokenized sample.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenizedSampleView<'a> {
    /// Borrowed token ids.
    pub input_ids: &'a [u32],
    /// Borrowed attention mask.
    pub attention_mask: &'a [u8],
    /// Optional borrowed sample id.
    pub id: Option<&'a str>,
}
