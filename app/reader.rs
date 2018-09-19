
use hell::combo::*;
use hell::parser::*;

// TODO escape shebang

pub fn default_reader() -> Reader {
    Reader::new("nothing", vec![

        ReaderArm::new("hash")
            .accept_input(Combo::Just('#'.into()))
            .accept_state(Combo::Just("hash-bang-comment".into())
                          .or(Combo::Just("string".into())).negate())
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::set_state("hash"))
        ,
        ReaderArm::new("hash-bang")
            .accept_state(Combo::Just("hash".into()))
            .accept_input(Combo::Just('!'.into()))
            .run(ReaderInstruction::set_state("hash-bang-comment"))
        ,
        ReaderArm::new("hash-not-bang")
            .accept_state(Combo::Just("hash".into()))
            .accept_input(Combo::Anything)
            .run(ReaderInstruction::set_state("nothing"))
        ,
        ReaderArm::new("hash-bang-comment-bang")
            .accept_state(Combo::Just("hash-bang-comment".into()))
            .accept_input(Combo::Just('!'.into()))
            .run(ReaderInstruction::set_state("hash-bang-comment-bang"))
        ,
        ReaderArm::new("hash-bang-comment")
            .accept_state(Combo::Just("hash-bang-comment".into()))
            .accept_input(Combo::Anything)
        ,
        ReaderArm::new("hash-bang-comment-bang-hash")
            .accept_state(Combo::Just("hash-bang-comment-bang".into()))
            .accept_input(Combo::Just('#'.into()))
            .run(ReaderInstruction::set_state("nothing"))
        ,
        ReaderArm::new("hash-bang-comment-bang-not-hash")
            .accept_state(Combo::Just("hash-bang-comment-bang".into()))
            .accept_input(Combo::Anything)
        ,

        ReaderArm::new("escape-string")
            .accept_input(Combo::Just('\\'.into()))
            .accept_state(Combo::Just("string".into()))
            .run(ReaderInstruction::set_state("string-escape"))
        ,
        ReaderArm::new("string-escaped-dquote")
            .accept_input(Combo::Just('"'.into()))
            .accept_state(Combo::Just("string-escape".into()))
            .run(ReaderInstruction::set_state("string"))
            .run(ReaderInstruction::append_token())
        ,
        ReaderArm::new("string-escaped-escape")
            .accept_input(Combo::Just('\\'.into()))
            .accept_state(Combo::Just("string-escape".into()))
            .run(ReaderInstruction::set_state("string"))
            .run(ReaderInstruction::append_token())
        ,
        ReaderArm::new("start-string")
            .accept_input(Combo::Just('"'.into()))
            .accept_state(Combo::Just("string".into()).negate())
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::start_token())
            .run(ReaderInstruction::set_tag("\"\""))
            .run(ReaderInstruction::set_type(TokenType::String))
            .run(ReaderInstruction::set_state("string"))
        ,
        ReaderArm::new("end-string")
            .accept_input(Combo::Just('"'.into()))
            .accept_state(Combo::Just("string".into()))
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::set_state("nothing"))
        ,
        ReaderArm::new("string-inner")
            .accept_input(Combo::Anything)
            .accept_state(Combo::Just("string".into()))
            .run(ReaderInstruction::append_token())
        ,

        ReaderArm::new("whitespace")
            .accept_input(Combo::Just(CharClass::Whitespace))
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::set_state("nothing"))
        ,

        ReaderArm::new("start-rational")
            .accept_input(Combo::Just(CharClass::Numeric))
            .accept_state(Combo::Just("nothing".into()))
            .run(ReaderInstruction::start_token())
            .run(ReaderInstruction::set_tag("rational"))
            .run(ReaderInstruction::set_type(TokenType::Rational))
            .run(ReaderInstruction::append_token())
            .run(ReaderInstruction::set_state("rational"))
        ,
        ReaderArm::new("rational")
            .accept_input(Combo::Just(CharClass::Numeric).or(Combo::Just('/'.into())))
            .accept_state(Combo::Just("rational".into()))
            .run(ReaderInstruction::append_token())
        ,

        ReaderArm::new("start-list")
            .accept_input(Combo::Just('['.into()))
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::start_token())
            .run(ReaderInstruction::set_tag("[]"))
            .run(ReaderInstruction::set_type(TokenType::StartList))
            .run(ReaderInstruction::append_token())
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::set_state("nothing"))
        ,
        ReaderArm::new("end-list")
            .accept_input(Combo::Just(']'.into()))
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::start_token())
            .run(ReaderInstruction::set_tag("[]"))
            .run(ReaderInstruction::set_type(TokenType::EndList))
            .run(ReaderInstruction::append_token())
            .run(ReaderInstruction::finish_token())
            .run(ReaderInstruction::set_state("nothing"))
        ,
        ReaderArm::new("end")
            .accept_input(Combo::Just(CharClass::Eof))
            // Should make sure it's not inside a string
            .accept_state(Combo::Just("string".into()).negate())
            .run(ReaderInstruction::finish_token())
        ,
        ReaderArm::new("start-atom")
            .accept_input(Combo::Anything)
            .accept_state(Combo::Just("nothing".into()))
            .run(ReaderInstruction::set_state("atom"))
            .run(ReaderInstruction::start_token())
            .run(ReaderInstruction::set_tag("word"))
            .run(ReaderInstruction::set_type(TokenType::Symbol))
            .run(ReaderInstruction::append_token())
        ,
        ReaderArm::new("atom")
            .accept_input(Combo::Anything)
            .accept_state(Combo::Just("atom".into()))
            .run(ReaderInstruction::append_token())
        ,
    ])
}


