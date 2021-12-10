import os
import time


class SortData():
    def __init__(self, size, default_sort_time, quick_sort_time, merge_sort_time):
        self.size = size
        self.default_sort_time = default_sort_time
        self.quick_sort_time = quick_sort_time
        self.merge_sort_time = merge_sort_time

    def __str__(self):
        return f"|{self.size} | {self.default_sort_time}ms | {self.quick_sort_time}ms | {self.merge_sort_time}ms |"


def read_all(file_name):
    data = []
    with open(file_name) as file:
        data = file.readlines()
        data = [int(dataum) for dataum in data]

    return data


def partition(input_list, start_index, end_index):
    pivot = input_list[end_index]
    i = start_index - 1

    for j in range(start_index, end_index + 1):
        if input_list[j] <= pivot:
            i += 1
            input_list[i], input_list[j] = input_list[j], input_list[i]

    return i


def do_quick_sort(input_list, start_index, end_index):
    if start_index >= end_index:
        return
    pivot = partition(input_list, start_index, end_index)

    do_quick_sort(input_list, start_index, pivot - 1)
    do_quick_sort(input_list, pivot + 1, end_index)


def quick_sort(input_list):
    last_index = len(input_list) - 1
    do_quick_sort(input_list, 0, last_index)


def merge(input_list, left, right):
    i = 0
    j = 0
    k = 0

    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            input_list[k] = left[i]
            i += 1
        else:
            input_list[k] = right[j]
            j += 1
        k += 1

    while i < len(left):
        input_list[k] = left[i]
        i += 1
        k += 1

    while j < len(right):
        input_list[k] = right[j]
        j += 1
        k += 1


def merge_sort(input_list):
    if len(input_list) > 1:
        mid = len(input_list) // 2
        left = input_list[:mid]
        right = input_list[mid:]

        merge_sort(left)
        merge_sort(right)

        merge(input_list, left, right)


def default_sort(input_list):
    input_list.sort()


def runtime_average(input_list, count, method):
    sum = 0
    for _ in range(0, count):
        input_clone = input_list.copy()
        start = time.process_time()
        method(input_clone)
        sum += time.process_time() - start

    return sum * 1000 / count


if __name__ == '__main__':
    directory = '../inputs'
    files = [os.path.join(directory, x) for x in os.listdir(directory)]
    sort_data_list = list()
    for file in files:
        data = read_all(file)
        default_sort_time = runtime_average(data, 10, default_sort)
        quick_sort_time = runtime_average(data, 10, quick_sort)
        merge_sort_time = runtime_average(data, 10, merge_sort)
        sort_data_list.append(SortData(len(data), default_sort_time, quick_sort_time, merge_sort_time))

    sort_data_list.sort(key=lambda x: x.size)
    for item in sort_data_list:
        print(item)
