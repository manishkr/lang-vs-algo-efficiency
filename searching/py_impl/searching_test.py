import os
import random
import unittest

import main


class SearchingTest(unittest.TestCase):
    def test_linear_search(self):
        directory = '../inputs'
        files = [os.path.join(directory, x) for x in os.listdir(directory)]
        for file in files:
            data = main.read_all(file)
            element = random.randint(1, len(data))
            index = main.linear_search(data, element)
            default_index = -1
            if element in data:
                default_index = data.index(element)
            self.assertEqual(default_index, index)

    def test_binary_search(self):
        directory = '../inputs'
        files = [os.path.join(directory, x) for x in os.listdir(directory)]
        for file in files:
            data = main.read_all(file)
            data = list(set(data))
            data.sort()
            element = random.randint(1, len(data))
            index = main.binary_search(data, element)
            default_index = -1
            if element in data:
                default_index = data.index(element)
            self.assertEqual(default_index, index)

    def test_binary_search_recursive(self):
        directory = '../inputs'
        files = [os.path.join(directory, x) for x in os.listdir(directory)]
        for file in files:
            data = main.read_all(file)
            data = list(set(data))
            data.sort()
            element = random.randint(1, len(data))
            index = main.binary_search_recursive(data, element)
            default_index = -1
            if element in data:
                default_index = data.index(element)
            self.assertEqual(default_index, index)


if __name__ == '__main__':
    unittest.main()
