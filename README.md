# Shuffle data
## About this program
A tiny program for shuffle data (images etc.)

This program is made using [Tauri](https://tauri.app/).

However, this program is only prototype.

## Prepare Excel file
Prepare Excel file as follows.

At least **filename** column is required.

| ID | filename     |
|:--:|:------------:|
|  1 | image001.png |
|  2 | image002.png |
|  3 | image003.png |
|  4 | image004.png |
|  5 | image005.png |
|  6 | image006.png |

## Shuffle data with this program

### Excel file and image files
Prepare Excel file and images.

For example
```
.
└── images
    ├── image001.png
    ├── image002.png
    ├── image003.png
    ├── image004.png
    ├── image005.png
    ├── image006.png
    ├── result         <= folder for output data
    └── sample.xlsx
```

### Run program
1. Select Excel file
    - set path to folder automaticaly
1. Select images / save folder, if necessary
1. Push "shuffle data" button
1. Output Excel file and shuffled image files are saved.

```
.
└── images
    ├── image001.png
    ├── image002.png
    ├── image003.png
    ├── image004.png
    ├── image005.png
    ├── image006.png
    ├── result         <= folder for output data
    │   ├── shuffled-sample.xlsx
    │   ├── shuffled_001.png
    │   ├── shuffled_002.png
    │   ├── shuffled_003.png
    │   ├── shuffled_004.png
    │   ├── shuffled_005.png
    │   └── shuffled_006.png
    └── sample.xlsx
```

### Output Excel file
| ID | filename     | new_filename     |
|:--:|:------------:|------------------|
|  5 | image005.png | shuffled_001.png |
|  4 | image004.png | shuffled_002.png |
|  1 | image001.png | shuffled_003.png |
|  3 | image003.png | shuffled_004.png |
|  6 | image006.png | shuffled_005.png |
|  2 | image002.png | shuffled_006.png |


## LICENSE
MIT LICENSE

## Author
ARA Toshiaki
