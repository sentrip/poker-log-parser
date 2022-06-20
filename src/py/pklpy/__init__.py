import pklp

__all__ = [
    "str_to_json",
    "path_to_json",
    "path_to_json_file",
    "strs_to_json",
    "paths_to_json",
    "paths_to_json_file",
]

def str_to_json(data: str) -> str:
    """
    Convert string into poker hand history JSON
    :param data: str: poker hand history string
    :return: str: poker hand history JSON
    """
    return pklp.str_to_json(data)


def path_to_json(path: str) -> str:
    """
    Convert contents of file at given path into poker hand history JSON
    :param path: str: path to file containing poker hand history text
    :return: str: poker hand history JSON
    """
    return pklp.path_to_json(path)


def path_to_json_file(path: str, output_path: str):
    """
    Convert contents of file at given path into poker hand history JSON and write JSON to given output_path
    :param path: str: path to file containing poker hand history text
    :param output_path: str: path of file where JSON output is written
    """
    pklp.path_to_json_file(path, output_path)


def strs_to_json(data: [str]) -> str:
    """
    Convert a list of strings into combined poker hand history JSON
    :param data: list: list of poker hand history string
    :return: str: poker hand history JSON
    """
    return pklp.strs_to_json(data)


def paths_to_json(paths: [str]) -> str:
    """
    Convert contents of files at given paths into combined poker hand history JSON
    :param paths: list: list of paths to files containing poker hand history texts
    :return: str: poker hand history JSON
    """
    return pklp.paths_to_json(paths)


def paths_to_json_file(paths: [str], output_path: str):
    """
    Convert contents of files at given paths into combined poker hand history JSON and write JSON to given output_path
    :param paths: list: list of paths to files containing poker hand history texts
    :param output_path: str: path of file where JSON output is written
    """
    pklp.paths_to_json_file(paths, output_path)
