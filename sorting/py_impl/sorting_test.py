import os
import unittest
import main


class SortingTest(unittest.TestCase):
    def test_quick_sort(self):
        directory = '../inputs'
        files = [os.path.join(directory, x) for x in os.listdir(directory)]
        for file in files:
            data = main.read_all(file)
            input_clone_1 = data.copy()
            input_clone_2 = data.copy()
            main.quick_sort(input_clone_1)
            input_clone_2.sort()
            self.assertListEqual(input_clone_1, input_clone_2)


if __name__ == '__main__':
    unittest.main()
