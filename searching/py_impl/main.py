import os
import time


class SearchData():
    def __init__(self, size, linear_search_time, binary_search_time):
        self.size = size
        self.linear_search_time = linear_search_time
        self.binary_search_time = binary_search_time

    def __str__(self):
        return f"|{self.size} | {self.linear_search_time}ms | {self.binary_search_time}ms |"


def read_all(file_name):
    data = []
    with open(file_name) as file:
        data = file.readlines()
        data = [int(dataum) for dataum in data]

    return data


def linear_search(input, element):
    element_index = -1
    for index, item in enumerate(input):
        if item == element:
            element_index = index
            break

    return element_index


def runtime_average(input_list, element, count, method):
    sum = 0
    for _ in range(0, count):
        input_clone = input_list.copy()
        start = time.process_time()
        method(input_clone, element)
        sum += time.process_time() - start

    return sum * 1000 / count


if __name__ == '__main__':
    directory = '../inputs'
    files = [os.path.join(directory, x) for x in os.listdir(directory)]
    search_data_list = list()
    for file in files:
        data = read_all(file)
        element = data[-1]
        linear_search_time = runtime_average(data, element, 10, linear_search)
        binary_search_time = 0
        search_data_list.append(SearchData(len(data), linear_search_time, binary_search_time))

    search_data_list.sort(key=lambda x: x.size)
    for item in search_data_list:
        print(item)
