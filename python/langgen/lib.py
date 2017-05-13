
import re
from collections import defaultdict
import random

TERMS = ".?!"
WORD = re.compile(r'\w[\w\']*|[%s]' % TERMS)


def parse_file(fname):
    doc = []
    with open(fname, 'rt') as f:
        for line in f:
            doc.extend(WORD.findall(line))
    return doc


def parse_string(s):
    doc = []
    for line in s.splitlines():
        doc.extend(WORD.findall(line))
    return doc


def create_trigrams(doc):

    source = zip(doc, doc[1:], doc[2:])
    starts = []
    trigrams = defaultdict(list)
    for (w1, w2, w3) in source:
        trigrams[(w1, w2)].append(w3)
        if w1 in TERMS:
            starts.append(w2)
    return starts, trigrams


def random_sentence(trigrams, starts, max_len=1000):
    assert max_len >= 1
    term = ''
    prev = '.'
    curr = random.choice(starts)
    sentence = [curr]
    count = 1
    while True:
        nxt = trigrams.get((prev, curr))
        if not nxt:
            break
        nxt = random.choice(nxt)
        if nxt in TERMS:
            term = nxt
            break
        count += 1
        if count >= max_len:
            break
        sentence.append(nxt)
        prev, curr = curr, nxt

    res = ' '.join(sentence)
    if term:
        res += term
    return res
