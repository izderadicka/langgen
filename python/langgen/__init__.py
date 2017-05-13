from .lib import parse_file, parse_string, create_trigrams, random_sentence

def generate(fname, sentences=1):
    doc = parse_file(fname)
    starts, trigrams = create_trigrams(doc)
    res = []
    for _i in range(sentences):
        res.append(random_sentence(trigrams, starts))

    return '\n'.join(res)