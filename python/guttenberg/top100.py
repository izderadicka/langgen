import requests
import bs4 as bs
import sys
import os.path
from urllib.parse import urljoin
import re

base_url = 'https://www.gutenberg.org/browse/scores/top'

START = re.compile(
    "\*\*\* ?START OF (THE|THIS)? ?PROJECT GUTENBERG EBOOK", re.UNICODE | re.IGNORECASE)
END = re.compile(
    "\*\*\* ?END OF (THE|THIS)? ?PROJECT GUTENBERG EBOOK|End of (the )?Project Gutenberg", re.UNICODE | re.IGNORECASE)


def main():
    path = sys.argv[1]
    if not os.path.exists(path):
        os.makedirs(path)
    pg = bs.BeautifulSoup(requests.get(base_url).text, "lxml")
    header = pg.find('h2', id='books-last30')
    books_list = header.find_next('ol')
    links = books_list.find_all('a')
    for i, link in enumerate(links):
        url = urljoin(base_url, link['href'])
        name = link.text
        name = re.sub(r'\(\d+\)', '', name)
        pg = bs.BeautifulSoup(requests.get(url).text, "lxml")
        link = pg.find(
            'a', "link", type=re.compile("text/plain"),  title="Download")
        if link:
            link = urljoin(base_url, link['href'])
            print(i + 1, name, link)
            text = requests.get(link).text
            lines = text.splitlines()
            started = False
            for line_no, line in enumerate(lines):
                if START.match(line):
                    started = True
                    break
            if not started:
                print('NO START TAG!')
                line_no = 0
            with open(os.path.join(path, name + '.txt'), 'wt') as f:
                for line in lines[line_no + 1:]:
                    if END.match(line):
                        break
                    f.write(line + '\n')

        else:
            print(i + 1, name, "No text")


if __name__ == '__main__':
    main()
