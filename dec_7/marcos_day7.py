import hashlib
from copy import deepcopy


def process_input(input2):
    """the dumb version for posteritiy"""
    out = []
    i = 0
    full_path = []
    all_paths = []

    while True and i < len(input2):
        line = input2[i]
        if line.startswith('$ cd'):
            try:
                _, new_dir = line.split('cd ')
            except ValueError:
                print(f'wtf {line}')
            new_dir = new_dir.strip()
            if new_dir != "..":
                myhash = '#'.join(full_path)
                myhash = hashlib.md5(myhash.encode('utf8')).hexdigest(
                )[:
                  5]  # because folder names are not unique nor is their depth so I need something that
                #says no I mean this version of this folder name
                full_path.append(new_dir + myhash)
                all_paths.append(new_dir + myhash)

            elif new_dir == "..":
                _ = full_path.pop()
        elif not line.startswith('$'):
            size = 0
            if line[0].isdigit():
                size, _ = line.split(' ')
                size = int(size)
            # I don't understand python because without deepcopy this list I kept appending just wouldn't work
            mydict = {
                'full_path': deepcopy(full_path),
                'full_path_string': '/'.join(full_path),
                'file': line,
                'size': size
            }
            out.append(mydict)
        i = i + 1
    all_paths = set(all_paths)
    return out, all_paths


def process_input2(input2):
    """less dumb"""
    out = []
    i = 0
    full_path = []
    all_paths = []

    while True and i < len(input2):
        line = input2[i]
        if line.startswith('$ cd'):
            try:
                _, new_dir = line.split('cd ')
            except ValueError:
                print(f'wtf {line}')
            new_dir = new_dir.strip()
            if new_dir != "..":  # because folder names are not unique nor is their depth so I need something that
                #says no I mean this version of this folder name
                full_path.append(new_dir)
                #all_paths.append(new_dir)

            elif new_dir == "..":
                _ = full_path.pop()
        elif not line.startswith('$'):
            size = 0
            if line[0].isdigit():
                size, _ = line.split(' ')
                size = int(size)
            # I don't understand python because without deepcopy this list I kept appending just wouldn't work
            mydict = {
                'full_path': deepcopy(full_path),
                'full_path_string': '/'.join(full_path),
                'file': line,
                'size': size
            }
            out.append(mydict)
            all_paths.append('/'.join(full_path))
        i = i + 1
    all_paths = set(all_paths)
    return out, all_paths
