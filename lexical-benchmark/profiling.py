#!/usr/bin/env python
'''
    profiling
    =========

    Create a baseline for metrics between various tools.

    This finds all the results from criterion in the target directories,
    and then concatenates them and joins various tooling into a single
    file.

    The file will be output to `/target/profiling.json` (by default).
'''

import argparse
import json
from collections import defaultdict
from pathlib import Path

home = Path(__file__).absolute().parent

parser = argparse.ArgumentParser(
    prog='Profiling',
    description='Compare profiling results between criterion runs.'
)
parser.add_argument(
    '-o',
    '--output',
    '--output-file',
    dest='output',
    type=Path,
    help='The file to save the report to.',
    default=home / 'target' / 'profiling.json',
)
parser.add_argument(
    '-p',
    '--profile',
    help='The name of the profile to load the results from.',
    default='base',
)
args = parser.parse_args()

# the structure is:
#   criterion -> group -> bench -> profile
# load all our files, and collate them by group and the like
criterion = home / 'target' / 'criterion'
files = criterion.rglob(f'*/*/{args.profile}/estimates.json')
results = defaultdict(lambda: defaultdict(dict))
for file in files:
    group = file.parent.parent.parent.name
    name = file.parent.parent.name
    with (criterion / file).open(encoding='utf-8') as fp:
        results[group][name] = json.load(fp)

# now we need to collate everything by groups, etc.
profiling = defaultdict(lambda: defaultdict(dict))
for group, items in results.items():
    for name, item in items.items():
        mean = item['mean']['point_estimate']
        lower = item['mean']['confidence_interval']['lower_bound']
        upper = item['mean']['confidence_interval']['upper_bound']
        profiling[group]['mean'][name] = mean
        profiling[group]['lower'][name] = lower
        profiling[group]['upper'][name] = upper

with open(args.output, 'w', encoding='utf-8') as fp:
    json.dump(profiling, fp, indent=2)
