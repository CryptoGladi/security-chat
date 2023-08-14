use super::*;
use ngrammatic::{CorpusBuilder, Pad, SearchResult};

impl<'a> Runner<'a> {
    pub fn get_fuzzy_array(&self, str: &str) -> Vec<SearchResult> {
        let mut corpus = CorpusBuilder::new()
            .pad_full(Pad::Auto)
            .finish();
        self.commands.keys().for_each(|&x| corpus.add_text(x));

        let id = str.split_whitespace().next().unwrap();
        corpus.search(id, 0.1)[..self.limit_fuzzy].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use test_log::test;

    #[test]
    fn get_fuzzy_array() {
        let runner = RunnerBuilder::default()
            .commands(vec![&TestCommand, &SameTestCommand])
            .limit_fuzzy(2)
            .build()
            .unwrap();

        let output = runner
            .get_fuzzy_array(TestCommand.get_id())
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
        .limit_fuzzy(1)
        .build()
        .unwrap();
    
        assert_eq!(runner.get_fuzzy_array("test_command").len(), 1);
    }
}
