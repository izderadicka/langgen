import unittest
from langgen.lib import *
from langgen import generate


TEST_TEXT = """
Say hello
hello you must say,
to all good men.
Don't say hello to bad men!"""


class Test(unittest.TestCase):
    def test_doc_parse(self):
        doc = parse_string(TEST_TEXT)
        print("DOC", doc)
        self.assertEqual(len(doc), 18)
        
        doc = parse_file(__file__)
        self.assertTrue(len(doc)> 18)

    def test_trigrams(self):
        doc = parse_string(TEST_TEXT)
        starts, trigrams = create_trigrams(doc)
        self.assertEqual(len(starts), 1)
        self.assertTrue(len(trigrams)>10)
        print(trigrams)


    def test_sentence(self):
        doc = parse_string(TEST_TEXT)
        starts, trigrams = create_trigrams(doc)
        sentence = random_sentence(trigrams, starts)
        self.assertEqual(sentence, "Don't say hello to bad men!")

    def test_generate(self):
        s = generate(__file__, 3)
        print(s)
        self.assertEqual(len(s.splitlines()), 3)
