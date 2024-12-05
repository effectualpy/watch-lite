## Setup

To install this just run:

    pip install watch-lite

## Usage

```python
import watch_lite

fileHash = watch_lite.getHash("filename.txt")

while True:
    if watch_lite.compareHash(lastHash):
        fileHash = watch_lite.getHash("filename.txt")
    
```
