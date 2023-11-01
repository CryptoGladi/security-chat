use super::*;
use ngrammatic::{CorpusBuilder, Pad, SearchResult};

const THRESHOLD: f32 = 0.1;

impl<'a> Runner<'a> {
    /// Searching command by [fuzzy](https://en.wikipedia.org/wiki/Approximate_string_matching)
    pub fn fuzzy_search(&self, command: &str) -> Vec<SearchResult> {
        if command.is_empty() {
            return vec![];
        }

        let mut corpus = CorpusBuilder::new().pad_full(Pad::Auto).finish();
        self.commands.keys().for_each(|&x| corpus.add_text(x));
        let id = command.split_whitespace().next().unwrap();

        let result = corpus.search(id, THRESHOLD);

        trace!("run `get_fuzzy_array`: {:?}", result);

        result
            .iter()
            .take(self.limit_for_fuzzy_search)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use test_log::test;

    #[test]
    fn fuzzy_search() {
        let runner = RunnerBuilder::default()
            .commands(vec![&TestCommand, &SameTestCommand])
            .limit_for_fuzzy_search(2)
            .build()
            .unwrap();

        let output = runner
            .fuzzy_search(TestCommand.get_id())
            .into_iter()
            .map(|x| x.text)
            .collect::<Vec<String>>();

        assert_eq!(
            output,
            vec!["test_command".to_string(), "same_test_command".to_string()]
        );
    }

    #[test]
    fn limit_fuzzy() {
        let runner = RunnerBuilder::default()
            .commands(vec![&TestCommand, &SameTestCommand])
            .limit_for_fuzzy_search(1)
            .build()
            .unwrap();

        assert_eq!(runner.fuzzy_search("test_command").len(), 1);
    }

    #[test]
    fn empty_get() {
        let runner = RunnerBuilder::default()
            .commands(vec![&TestCommand])
            .limit_for_fuzzy_search(10)
            .build()
            .unwrap();

        assert!(runner.fuzzy_search("1").is_empty());
    }

    #[test]
    fn empty_str() {
        let runner = RunnerBuilder::default()
            .commands(vec![&TestCommand])
            .limit_for_fuzzy_search(1)
            .build()
            .unwrap();

        assert!(runner.fuzzy_search("").is_empty());
    }
}
