use std::{sync::OnceLock, fs::File, io::{Read, Write as IoWrite}, fmt::Write as FmtWrite};
use regex::Regex;

pub const MAJOR_PART_PATTERN: &str = r"(?x) # Enable verbose mode (ignores whitespace in pattern)
    ^\s*\#\#\#\#*\n                     # Match initial #### pattern line
    \s*                                 # Optional whitespace
    (.+?)                               # Capture the title (non-greedy)
    \s*                                 # Optional whitespace
    \#\#\#\#*                           # Match ending #### pattern line
    ";

pub const SECTION_PATTERN: &str = r"(?x) # Enable verbose mode (ignores whitespace in pattern)
    ^\s*\#\*\#\*.*\n                 # Match initial #*#* pattern line
    \s*                              # Optional whitespace
    (.+?)                            # Capture the title (non-greedy)
    \s*                              # Optional whitespace
    \#\*\#\*                         # Match ending #*#* pattern line
    ";

pub const SUBSECTION_PATTERN: &str = r"(?x) # Enable verbose mode (ignores whitespace in pattern)
    ^\s*\=\-\=\-.*\n                    # Match initial =-=- pattern line
    \s*                                 # Optional whitespace
    (.+?)                               # Capture the title (non-greedy)
    \s*                                 # Optional whitespace
    \=\-\=\-                            # Match ending =-=- pattern line
    ";

pub const SUBSUBSECTION_PATTERN: &str = r"(?x) # Enable verbose mode (ignores whitespace in pattern)
    ^\s*\-\.\-\..*\n                       # Match initial -.-. pattern line
    \s*                                    # Optional whitespace
    (.+?)                                  # Capture the title (non-greedy)
    \s*                                    # Optional whitespace
    \-\.\-\.                               # Match ending -.-. pattern line
    ";

pub fn extract_major_part(text: &str) -> Option<String> {
    static MAJOR_PART_MATCH: OnceLock<Regex> = OnceLock::new();
    if MAJOR_PART_MATCH.get_or_init(|| {
        Regex::new(MAJOR_PART_PATTERN).unwrap()
    }).is_match(text) {
        let re = Regex::new(MAJOR_PART_PATTERN).unwrap();
        re.captures(text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
    } else { None }
}

pub fn extract_section(text: &str) -> Option<String> {
    static SECTION_MATCH: OnceLock<Regex> = OnceLock::new();
    if SECTION_MATCH.get_or_init(|| {
        Regex::new(SECTION_PATTERN).unwrap()
    }).is_match(text) {
        let re = Regex::new(SECTION_PATTERN).unwrap();
        re.captures(text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
    } else { None }
}

pub fn extract_sub_section(text: &str) -> Option<String> {
    static SUBSECTION_MATCH: OnceLock<Regex> = OnceLock::new();
    if SUBSECTION_MATCH.get_or_init(|| {
        Regex::new(SUBSECTION_PATTERN).unwrap()
    }).is_match(text) {
        let re = Regex::new(SUBSECTION_PATTERN).unwrap();
        re.captures(text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
    } else { None }
}

pub fn extract_sub_sub_section(text: &str) -> Option<String> {
    static SUBSUBSECTION_MATCH: OnceLock<Regex> = OnceLock::new();
    if SUBSUBSECTION_MATCH.get_or_init(|| {
        Regex::new(SUBSUBSECTION_PATTERN).unwrap()
    }).is_match(text) {
        let re = Regex::new(SUBSUBSECTION_PATTERN).unwrap();
        re.captures(text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
    } else { None }
}

pub fn find_and_write_title_tree(file_path: &str) {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lookbehind = b' ';
    let mut comment = Vec::new();
    let mut is_comment = false;
    let mut idx = 0;
    let mut stack = vec![vec![vec![vec![()]]]];
    let mut last_level = 0;

    let mut o = String::new();

    for (i, &b) in contents.as_bytes().into_iter().enumerate() {
        if lookbehind == b'$' && b == b')' {
            is_comment = false;
            comment.pop();
            let s = String::from_utf8_lossy(&comment);
            if let Some(title) = extract_major_part(&s) {
                let ord = format!("{}.", stack.len());
                o.write_str(format!("{ord} {} $ @[{idx}(byte)]\n", title).as_str()).unwrap();
                stack.push(vec![vec![vec![()]]]);
                last_level = 1;
            } else if let Some(title) = extract_section(&s) {
                let ord = format!("{}.{}.", stack.len() - 1, stack.last().unwrap().len());
                o.write_str(format!("  {ord} {} $ @[{idx}(byte)]\n", title).as_str()).unwrap();
                stack.last_mut().unwrap().push(vec![vec![()]]);
                last_level = 2;
            } else if let Some(title) = extract_sub_section(&s) {
                let ord = format!("{}.{}.{}.", stack.len() - 1, stack.last().unwrap().len() - 1, stack.last().unwrap().last().unwrap().len());
                o.write_str(format!("    {ord} {} $ @[{idx}(byte)]\n", title).as_str()).unwrap();
                stack.last_mut().unwrap().last_mut().unwrap().push(vec![()]);
                if last_level == 1 { println!("{ord} has 2 level difference.") }
                last_level = 3;
            } else if let Some(title) = extract_sub_sub_section(&s) {
                let ord = format!("{}.{}.{}.{}.", stack.len() - 1, stack.last().unwrap().len() - 1, stack.last().unwrap().last().unwrap().len() - 1, stack.last().unwrap().last().unwrap().last().unwrap().len());
                o.write_str(format!("      {ord} {} $ @[{idx}(byte)]\n", title).as_str()).unwrap();
                stack.last_mut().unwrap().last_mut().unwrap().last_mut().unwrap().push(());
                if last_level <= 2 { println!("{ord} has 2~3 level difference.") }
                last_level = 4;
            }
            comment.clear();
        }
        if is_comment {
            comment.push(b);
        }
        if lookbehind == b'$' && b == b'(' {
            is_comment = true;
            idx = i - 1;
        }
        lookbehind = b;
    }

    let output_file_name = format!("../{}-title-tree.txt", file_path.split('/').last().unwrap());
    let mut output_file = File::create(output_file_name).unwrap();
    output_file.write(&o.as_bytes()).unwrap();
}