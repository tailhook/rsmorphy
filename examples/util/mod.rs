use rsmorphy::prelude::*;

pub fn print_row_parsed(morph: &MorphAnalyzer, n: usize, &Parsed { ref lex, score }: &Parsed) {
    println!(
        r#"{num:2}. {score:7.5} :: {lex:20} — {norm:20} :: {enc:35} :: {tag:40}"#,
        num = n + 1,
        score = score.value(),
        lex = lex.get_word(),
        norm = lex.get_normal_form(morph),
        tag = lex.get_tag(morph).string.as_str(),
        enc = lex.stack.encoded()
    );
}

pub fn print_row_lex(morph: &MorphAnalyzer, n: usize, lex: &Lex) {
    println!(
        r#"{num:2}. {score:7.5} :: {lex:20} — {norm:20} :: {enc:35} :: {tag:40}"#,
        num = n + 1,
        score = Score::Fake(0.0).value(), //lex.score.value(),
        lex = lex.get_word(),
        norm = lex.get_normal_form(morph),
        tag = lex.get_tag(morph).string.as_str(),
        enc = lex.stack.encoded()
    );
}
