# Partman - Part Manager for KiCad

## Overview

Partman manages adding new entries to a KiCad DB library.
It does not manage the symbol and footprint files, rather the database that KiCad uses.
The KiCad DB library directory requires a 'partman.toml' config file, run the partman executable within the directory.
Adding parts uses a Git commit inspired workflow, where an external editor is opened to edit the part details, once this file is saved and closed it will save the new part.

## Workflows

```bash
# Set shorthand for command
alias pm='path/to/partman'

# Create a new Partman config file in current directory
pm init

# Add a new part from example and re-build db
pm add

# Add a new part from DigiKey and re-build db
pm add --digikey "296-6501-1-ND"

# Add a new part from DigiKey and don't re-build db
pm add --digikey "296-6501-1-ND" --no-build

# Build database into SQLite file for KiCad
pm build

# Resume last session
pm resume

```

Library Reference  format: <LibraryNickname>:<SymbolName>