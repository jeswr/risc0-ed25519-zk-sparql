use oxrdf::Dataset;
use oxttl::NQuadsParser;
use std::collections::HashMap;
use oxrdf::{BlankNode, GraphName, Quad, Subject, Term};
use crate::verify::VerifyInput;
use crate::verify::ed25519_verify;

pub fn load_dataset(verify_inputs: Vec<VerifyInput>) -> (Vec<String>, Dataset) {
    let mut pub_keys: Vec<String> = Vec::new();
    let mut dataset: Dataset = Dataset::new();

    let mut i: u128 = 0;

    for input in verify_inputs {
        let mut map = HashMap::<String, u128>::new();

        // This takes approx 95 seconds for the VCs that we have
        for triple in NQuadsParser::new().for_reader(input.canonical_document.clone().as_bytes()) {
            let t1 = triple.expect("Failed to parse triple");

            // If graph_name is not default_graph, error
            if !t1.graph_name.is_default_graph() {
              panic!("Graph name is not default_graph");
            }

            let mut subject = t1.subject;
            let predicate = t1.predicate;
            let mut object = t1.object;

            if let Subject::BlankNode(blank_node) = subject {
              let term_string = blank_node.into_string();
              let id: u128;
              if !map.contains_key(&term_string) {
                id = i;
                map.insert(term_string, id);
                i += 1;
                if i > 1000000000000000000 {
                  panic!("i is too large");
                }
              } else {
                id = map[&term_string];
              }
              subject = Subject::BlankNode(BlankNode::new_from_unique_id(id));
            }

            if let Term::BlankNode(blank_node) = object {
              let term_string = blank_node.into_string();
              let id: u128;
              if !map.contains_key(&term_string) {
                id = i;
                map.insert(term_string, id);
                i += 1;
                if i > 1000000000000000000 {
                  panic!("i is too large");
                }
              } else {
                id = map[&term_string];
              }
              object = Term::BlankNode(BlankNode::new_from_unique_id(id));
            }

            let quad = Quad::new(subject, predicate, object, GraphName::DefaultGraph);
            dataset.insert(&quad);
        }

        pub_keys.push(input.public_key.clone());
        ed25519_verify(input).expect("Signature verification failed");
    }

    return (pub_keys, dataset)
}
