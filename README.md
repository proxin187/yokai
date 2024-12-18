# yokai

yokai is a manual tiling window manager where each window is represented as a leaf node of a [full binary tree](https://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees) and tiled with [binary space partitioning](https://en.wikipedia.org/wiki/Binary_space_partitioning).

## Features
 - Flexible manual window layouts, preselect or change split direction, insertion point and ratio
 - Keyboard driven, (mouse is optional)
 - Small and consise code base.

## Roadmap
 - [X] Configuration over socket architecture
 - [X] EWMH Support
 - [ ] Query Tree on startup
 - [ ] Advanced node selection

## Installing

yokai can be installed for any display manager by running `install.sh` and writing the following to `/usr/share/xsessions/yokai.desktop`:

```
[Desktop Entry]
Name=yokai
Comment=yokai window manager
Exec=yokai
Type=Application
```

## License

yokai is licensed under the MIT License.

