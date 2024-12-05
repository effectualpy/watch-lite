from watch_lite._core import get_hash, compare_hash, get_all_hashes


def getHash(filePath: str) -> str:
    """
    Gets a file's current hash
    """
    return get_hash(filePath)

def compareHash(filePath: str, previousHash: str) -> bool:
    """
    Checks a file's previous hash against its 
    current one
    """
    return compare_hash(filePath, previousHash)

def getAllHashes(directoryPath: str) -> set[str]:
    """
    Gets all the hashes in a folder and places
    them into a set
    """
    return get_all_hashes(directoryPath)

__all__ = ["compareHash", "getHash", "getAllHashes"]
