use oxrdf::Dataset;
use sparesults::{QueryResultsFormat, QueryResultsSerializer};
use spareval::{QueryEvaluator, QueryResults};
use spargebra::Query;

pub fn query(dataset: Dataset, query: Query) -> String {
    let results = QueryEvaluator::new().execute(dataset, &query);

    // let size = dataset.len();

    if let Ok(QueryResults::Solutions(solutions)) = results {
        let serializer = QueryResultsSerializer::from(QueryResultsFormat::Json);
        let mut writer = Vec::new();

        let mut res = serializer
            .serialize_solutions_to_writer(&mut writer, solutions.variables().to_vec()).expect("Failed to serialize solutions");

        for solution in solutions {
            if let Ok(solution) = solution {
                let _ = res.serialize(solution.iter().map(|v| v.clone()).collect::<Vec<_>>());
            }
        }

        res.finish().unwrap();
        String::from_utf8(writer).unwrap()
    } else {
        String::new()
    }
}
