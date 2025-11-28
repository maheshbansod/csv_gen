# csvgen

A scalable CSV generator with precise size and row control.

## Features

- Generate CSV files by target size (KB, MB, GB) and row count
- Intelligent column distribution with unique headers
- Configurable column constraints
- Unique first column with sequential IDs
- Email and domain column generation with realistic data
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

# With email and domain columns
csvgen --size 1MB --rows 5000 --email-columns 3 --domain-columns 2 --output mixed.csv

# Size formats supported: KB, MB, GB, B
csvgen --size 500KB --rows 1000 --output small.csv
```

## Options

- `--size`: Target file size (e.g., 1MB, 500KB, 2GB)
- `--rows`: Number of rows to generate
- `--output`: Output file path (default: output.csv)
- `--min-columns`: Minimum number of columns (default: 2)
- `--max-columns`: Maximum number of columns (default: 100)
- `--email-columns`: Number of email columns to generate (default: 0)
- `--domain-columns`: Number of domain columns to generate (default: 0)

## How It Works

The generator intelligently distributes bytes across columns:
1. Calculates target row size from total size ÷ rows
2. Optimizes column count within constraints
3. Allocates space for unique headers and separators
4. Generates data with unique first column (sequential IDs)
5. Achieves precise file size targeting (within 1-2%)

## Example Output

### Standard CSV
```csv
idxxxxxxxxxxxxxxxxxxxxxxxxxxx,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z
0001,0hgElXw02Q0wC0,lXqTmUrrHXT4UL,FtdJVJIcvbN1jQ,Lg4qObtpFDw8Ex
0002,b9IxYa6lWtPY1E,M3ysdnwpOA67dY,Q7UVmYUpDTC5tb,gj8M9AAV1thYh8
```

### CSV with Email and Domain Columns
```csv
idxxxxxxxxxxxxxxxx,email_1xxxxxxx,email_2xxxxxxx,domain_1xxxxxx,col5eeeeeeeeee,col6ffffffffff
000000000000000001,mknaa@test.com,vs@example.org,app.dev,6X1rGLQfFFLlRK,3Sq8AeYa5xEUqy
000000000000000002,gnopv@demo.net,owvtv@test.com,mail.app.dev,9mSMroe1KNtxQt,URdwRZbCR4vY9u
```

### Column Types
- **Email columns**: Generate valid email addresses (e.g., `user@test.com`)
- **Domain columns**: Generate realistic domain names (e.g., `www.example.com`)
- **Standard columns**: Generate random alphanumeric strings
- **ID column**: Sequential unique identifiers (first column)
