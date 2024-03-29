use crate::analysis::{types::{TilStreamingInterface, TilSignal, streaming_interface::{Generic, GenericType, TilStreamDirection}, stream_types::StreamTypeDecl}, GeneratorParams, analyzer::{type_manager::StreamType, file_manager::TemplateType}};

use super::{JsonComponent, Matcher, Generatable, JsonComponentValue};

impl Matcher {
    pub fn new(name: &str, holder_name: &str, matcher: String, outer_nested: usize) -> Matcher {
        Matcher {
            name: name.to_string(),
            holder_name: holder_name.to_string(),
            matcher,
            outer_nested
        }
    }

    pub fn get_matcher(&self) -> &str {
        &self.matcher
    }
}

impl Generatable for Matcher {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::default();

        interface.add_generic(Generic::new("BPC", GenericType::Positive(gen_params.epc)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input, 
            StreamTypeDecl::new(
                StreamType::MatcherStr,
                None
            )
        );

        // Output type
        interface.add_stream("output", TilStreamDirection::Output,  
            StreamTypeDecl::new(
                StreamType::MatcherMatch,
                None
            )
        );

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::MatcherStr, StreamType::MatcherMatch]
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_outgoing_signals(&self) -> Vec<TilSignal> {
        vec![
            TilSignal::Intermediate { 
                source_inst_name: self.get_instance_name(), 
                source_stream_name: "output".to_owned(), 
                dest_inst_name: format!("{}_inst", self.holder_name), 
                dest_stream_name: "matcher_match".to_owned() 
            }
        ]
    }

    fn num_outgoing_signals(&self) -> usize {
        1
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_file_type(&self) -> TemplateType {
        TemplateType::Matcher(self.matcher.clone())
    }
}

impl JsonComponentValue for Matcher {
    fn to_graph_node(&self) -> String {
        format!("Regex matcher\n\"{}\"", self.matcher)
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        Vec::new()
    }

    fn num_children(&self) -> usize {
        0
    }
}