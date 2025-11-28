# csvgen

A scalable CSV generator with precise size and row control.

## Features

- Generate CSV files by target size (KB, MB, GB) and row count
- Intelligent column distribution with unique headers
- Configurable column constraints
- Unique first column with sequential IDs
- Progress reporting for large files

## Installation

```bash
cargo install --git https://github.com/maheshbansod/csv_gen
```

## Usage

```bash
# Basic usage
csvgen --size 1MB --rows 5000 --output data.csv

# With column constraints
csvgen --size 1MB --rows 5000 --min-columns 50 --output data.csv

# Size formats supported: KB, MB, GB, B
csvgen --size 500KB --rows 1000 --output small.csv
```

## Options

- `--size`: Target file size (e.g., 1MB, 500KB, 2GB)
- `--rows`: Number of rows to generate
- `--output`: Output file path (default: output.csv)
- `--min-columns`: Minimum number of columns (default: 2)
- `--max-columns`: Maximum number of columns (default: 100)

## How It Works

The generator intelligently distributes bytes across columns:
1. Calculates target row size from total size ÷ rows
2. Optimizes column count within constraints
3. Allocates space for unique headers and separators
4. Generates data with unique first column (sequential IDs)
5. Achieves precise file size targeting (within 1-2%)

## Example Output

```csv
idxxxxxxxxxxxxxxxxxxxxxxxxxxx,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z
0001,0hgElXw02Q0wC0,lXqTmUrrHXT4UL,FtdJVJIcvbN1jQ,Lg4qObtpFDw8Ex
0002,b9IxYa6lWtPY1E,M3ysdnwpOA67dY,Q7UVmYUpDTC5tb,gj8M9AAV1thYh8
```
