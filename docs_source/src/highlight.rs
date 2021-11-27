use itertools::Itertools;
use proc_macro2::LineColumn;
use syn::*;

pub fn highlight(code: &str) -> String {
    // it's an approximate lower-bound, since we'll add some amount of text with tags
    let mut output = String::with_capacity(code.len() * 2);
    output.push_str("<pre>\n");
    let file: File = match parse_file(code) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error parsing code: {:?}", e);
            return String::from(code);
        }
    };
    highlight_file(file, &mut output);

    output + "\n</pre>"
}

fn escape(input: impl Into<String>) -> String {
    input
        .into()
        .chars()
        .flat_map::<Vec<char>, _>(|ch| match ch {
            '>' => "&gt;".chars().collect(),
            '<' => "&lt;".chars().collect(),
            '&' => "&amp;".chars().collect(),
            '\'' => "&#39;".chars().collect(),
            '"' => "&quot;".chars().collect(),
            ch => vec![ch],
        })
        .collect()
}

fn whitespace(output: &mut String, cur: &LineColumn, next: LineColumn) -> LineColumn {
    (cur.line..next.line).for_each(|_| output.push('\n'));
    (0..next.column).for_each(|_| output.push(' '));
    next
}

fn highlight_file(input: File, output: &mut String) {
    let mut pos = LineColumn { line: 1, column: 0 };
    if let Some(shebang) = input.shebang {
        output.push_str(&escape(shebang));
        pos.line = 2;
    }
    input
        .attrs
        .iter()
        .for_each(|attr| highlight_attr(attr, output, &mut pos));
}

fn highlight_attr(input: &Attribute, output: &mut String, pos: &mut LineColumn) {
    *pos = whitespace(output, pos, input.pound_token.span.start());
    output.push_str("<span class=\"attribute\">#");
    output.push_str(match input.style {
        AttrStyle::Inner(_) => "!",
        AttrStyle::Outer => "",
    });
    output.push_str("[");
    output.push_str(&process_path(&input.path));
    output.push_str("]</span>");
    *pos = input.bracket_token.span.end();
}

fn process_path(input: &Path) -> String {
    let mut output = match input.leading_colon {
        Some(_) => "::",
        None => "",
    }
    .to_string();
    output.extend(
        input
            .segments
            .iter()
            .map(process_path_segment)
            .join("::")
            .chars(),
    );
    output
}

fn process_path_segment(input: &PathSegment) -> String {
    use syn::PathArguments::*;
    let mut output = input.ident.to_string();
    match &input.arguments {
        None => {}
        AngleBracketed(args) => output.push_str(&process_angle_generic(args)),
        Parenthesized(args) => output.push_str(&process_fn_generic_argument(args)),
    };
    output
}

fn process_angle_generic(input: &AngleBracketedGenericArguments) -> String {
    let mut output = match input.colon2_token {
        Some(_) => "::<",
        None => "<",
    }
    .to_string();
    output.extend(
        input
            .args
            .iter()
            .map(process_generic_argument)
            .join("::")
            .chars(),
    );
    output + ">"
}

fn process_generic_argument(input: &GenericArgument) -> String {
    use syn::GenericArgument::*;
    match input {
        Lifetime(lt) => process_lifetime(lt),
        Type(ty) => process_type(ty),
        Binding(bind) => process_binding(bind),
        Constraint(_) => unimplemented!(),
        Const(_) => unimplemented!(),
    }
}

fn process_lifetime(input: &Lifetime) -> String {
    format!("<span class=\"lifetime\">'{}</span>", input.ident.to_string())
}

fn process_binding(input: &Binding) -> String {
    format!("{} = {}", process_ident(&input.ident), process_type(&input.ty))
}

fn process_ident(input: &Ident) -> String {
    format!("<span class=\"ident\">{}</span>", input)
}

fn process_type(input: &Type) -> String {
    use syn::Type::*;
    match input {
        Array(_) => unimplemented!(),
        BareFn(_) => unimplemented!(),
        Group(_) => unimplemented!(),
        ImplTrait(_) => unimplemented!(),
        Infer(_) => unimplemented!(),
        Macro(_) => unimplemented!(),
        Never(_) => unimplemented!(),
        Paren(_) => unimplemented!(),
        Path(_) => unimplemented!(),
        Ptr(_) => unimplemented!(),
        Reference(_) => unimplemented!(),
        Slice(_) => unimplemented!(),
        TraitObject(_) => unimplemented!(),
        Tuple(_) => unimplemented!(),
        Verbatim(_) => unimplemented!(),
        _ => panic!("Type not implemented, check syn docs"),
    }
}

fn process_fn_generic_argument(input: &ParenthesizedGenericArguments) -> String {
    let list = input.inputs.iter().map(process_type).join(", ");
    format!("({}){}", list, process_return_type(&input.output))
}

fn process_return_type(input: &ReturnType) -> String {
    use syn::ReturnType::*;
    match input {
        Default => "".to_string(),
        Type(_, ty) => format!(" -> {}", process_type(&*ty))
    }
}