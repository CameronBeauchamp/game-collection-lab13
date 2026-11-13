# Task 1: Hand-Trace of the Blinker (Rows 3–7, Columns 8–12)

Rules of Conway's Game of Life:
- A live cell (`#`) with **2 or 3** live neighbors stays alive (`#`).
- A dead cell (`.`) with **exactly 3** live neighbors becomes alive (`#`).
- Every other cell is dead (`.`) in the next generation.

---

## Generation 0 (Starting Pattern)
Live cells: `(4, 10)`, `(5, 10)`, `(6, 10)`

### Grid (Generation 0)

| Row \ Col | 8 | 9 | 10 | 11 | 12 |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **3** | . | . | . | . | . |
| **4** | . | . | # | . | . |
| **5** | . | . | # | . | . |
| **6** | . | . | # | . | . |
| **7** | . | . | . | . | . |

### Live Neighbor Counts for Generation 0

| Row \ Col | 8 | 9 | 10 | 11 | 12 |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **3** | 0 | 1 | 1 | 1 | 0 |
| **4** | 0 | 2 | 1 | 2 | 0 |
| **5** | 0 | 3 | 2 | 3 | 0 |
| **6** | 0 | 2 | 1 | 2 | 0 |
| **7** | 0 | 1 | 1 | 1 | 0 |

---

## Generation 1 (After 1 Step)
- `(4, 10)` and `(6, 10)` were live with 1 neighbor -> die (`.`).
- `(5, 10)` was live with 2 neighbors -> stays alive (`#`).
- `(5, 9)` and `(5, 11)` were dead with 3 neighbors -> become alive (`#`).

### Grid (Generation 1)

| Row \ Col | 8 | 9 | 10 | 11 | 12 |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **3** | . | . | . | . | . |
| **4** | . | . | . | . | . |
| **5** | . | # | # | # | . |
| **6** | . | . | . | . | . |
| **7** | . | . | . | . | . |

### Live Neighbor Counts for Generation 1

| Row \ Col | 8 | 9 | 10 | 11 | 12 |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **3** | 0 | 0 | 0 | 0 | 0 |
| **4** | 1 | 2 | 3 | 2 | 1 |
| **5** | 1 | 1 | 2 | 1 | 1 |
| **6** | 1 | 2 | 3 | 2 | 1 |
| **7** | 0 | 0 | 0 | 0 | 0 |

---

## Generation 2 (After 2 Steps)
- `(5, 9)` and `(5, 11)` were live with 1 neighbor -> die (`.`).
- `(5, 10)` was live with 2 neighbors -> stays alive (`#`).
- `(4, 10)` and `(6, 10)` were dead with 3 neighbors -> become alive (`#`).

### Grid (Generation 2)

| Row \ Col | 8 | 9 | 10 | 11 | 12 |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **3** | . | . | . | . | . |
| **4** | . | . | # | . | . |
| **5** | . | . | # | . | . |
| **6** | . | . | # | . | . |
| **7** | . | . | . | . | . |

### Live Neighbor Counts for Generation 2

| Row \ Col | 8 | 9 | 10 | 11 | 12 |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **3** | 0 | 1 | 1 | 1 | 0 |
| **4** | 0 | 2 | 1 | 2 | 0 |
| **5** | 0 | 3 | 2 | 3 | 0 |
| **6** | 0 | 2 | 1 | 2 | 0 |
| **7** | 0 | 1 | 1 | 1 | 0 |
