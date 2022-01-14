import os
import time


class SearchData():
    def __init__(self, size, linear_search_time, binary_search_time, binary_search_recursive_time):
        self.size = size
        self.linear_search_time = linear_search_time
        self.binary_search_time = binary_search_time
        self.binary_search_recursive_time = binary_search_recursive_time

    def __str__(self):
        return f"|{self.size} | {self.linear_search_time}ms | {self.binary_search_time}ms | " \
               f"{self.binary_search_recursive_time}ms |"


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


def binary_search(input, element):
    start = 0
    end = len(input)

    while start < end:
        mid = int(start + (end - start) / 2)
        mid_element = input[mid]
        if mid_element > element:
            end = mid
        elif mid_element < element:
            start = mid + 1
        else:
            return mid

    return -1


def do_binary_search_recursive(input, start, end, element):
    if end < start:
        return -1

    mid = int(start + (end - start) / 2)

    if input[mid] > element:
        return do_binary_search_recursive(input, start, mid - 1, element)
    elif input[mid] < element:
        return do_binary_search_recursive(input, mid + 1, end, element)
    else:
        return mid


def binary_search_recursive(input, element):
    return do_binary_search_recursive(input, 0, len(input) - 1, element)


def runtime_average(input_list, element, count, method):
    sum = 0
    for _ in range(0, count):
        start = time.process_time()
        method(input_list, element)
        sum += time.process_time() - start

    return sum * 1000 / count


if __name__ == '__main__':
    directory = '../inputs'
    files = [os.path.join(directory, x) for x in os.listdir(directory)]
    search_data_list = list()
    for file in files:
        data = read_all(file)
        data.sort()
        element = data[-1]
        linear_search_time = runtime_average(data, element, 10, linear_search)
        binary_search_time = runtime_average(data, element, 10, linear_search)
        binary_search_recursive_time = runtime_average(data, element, 10, linear_search)
        search_data_list.append(SearchData(len(data), linear_search_time, binary_search_time, binary_search_recursive_time))

    search_data_list.sort(key=lambda x: x.size)
    for item in search_data_list:
        print(item)
