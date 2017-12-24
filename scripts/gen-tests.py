import os

from os import path

prelude = '''
extern crate jsish_rust as jsish;

use std::fs::File;
use std::io::prelude::*;
use std::fmt::Write;

use jsish::types::JsishError;
use jsish::parser::parse;
'''

parser_template = '''
#[test]
fn {}() {{
    let mut out_string = String::new();

    let mut outfile = File::open("{}").expect("Bad Output Filename");

    outfile.read_to_string(&mut out_string).expect("Couldn't read outfile");

    match parse("{}") {{
        Err(JsishError::Message(s)) => assert_eq!(s, out_string.trim()),
        _ => assert!(false)
    }}
}}
'''

print_ast_template = '''
#[test]
fn {}() {{
    let mut in_string = String::new();
    let mut out_string = String::new();

    let mut outfile = File::open("{}").expect("Bad Output Filename");

    outfile.read_to_string(&mut out_string).expect("Couldn't read outfile");

    let prog = parse("{}").expect("Bad Parse");

    write!(&mut in_string, "{{}}", prog).expect("Write Failed");

    assert_eq!(in_string.trim(), out_string.trim());
}}
'''

def group_test_paths(test_paths):

    corrects = [p for p in test_paths if path.splitext(p)[1] == '.correct']
    inputs = [p for p in test_paths if path.splitext(p)[1] == '.jsish']

    assert len(corrects) == len(inputs)

    matched_paths = zip(sorted(corrects), sorted(inputs))

    matched_paths = [(c, i) for (c,i) in matched_paths
                     if path.splitext(c)[0] == path.splitext(i)[0]]

    assert len(corrects) == len(matched_paths)
    assert len(inputs) == len(matched_paths)

    return matched_paths

def gen_test_entries(test_type, folder_name, hw_dir):

    parent_path = path.join(hw_dir.path, folder_name)
    if path.isdir(parent_path):
        test_paths = [entry.path for entry in os.scandir(parent_path)]
        test_groups = group_test_paths(test_paths)
        it_entries = [("{}_{}_{:02d}".format(hw_dir.name, test_type, i + 1), j, o) 
                      for (i, (j, o)) in enumerate(test_groups)]
        return it_entries
    else:
        return []

os.chdir('../')

hw_dirs = [p for p in os.scandir('tests') if p.is_dir()]

it_names = {'parser': [], 'print_ast': [], 'eval': [], 'type_error': [], 'gc': []}

for hw_dir in hw_dirs:

        it_names['parser'] += gen_test_entries('parser', '1_parser', hw_dir)
        it_names['print_ast'] += gen_test_entries('print_ast', '2_3_ast_echo', hw_dir)

print(prelude)

for name, outfile, infile in it_names['parser']:
    print(parser_template.format(name, outfile, infile))

for name, outfile, infile in it_names['print_ast']:
    print(print_ast_template.format(name, outfile, infile))
