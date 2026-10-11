# Function parity: worksheet functions and formulas

> **Last reviewed:** 2026-10-11 · **Last updated:** 2026-10-11 · **Change:** minor (ODDFPRICE, ODDFYIELD, ODDLPRICE, ODDLYIELD added) · **Target:** Microsoft Excel (Microsoft 365), Excel for Mac 16.113.4

Excel's worksheet functions, category by category, against GridCraft's function library. Part of
[target-app-parity.md](target-app-parity.md); gaps feed [gaps.md](gaps.md).

## How this is measured

- **Excel's list (523):** Microsoft's "Excel functions (alphabetical)" support page, fetched and
  parsed on 2026-10-10 (506 rows with their category and the Excel version that introduced
  them), plus 14 text functions the page lists under combined entries (FIND/FINDB, LEFT/LEFTB,
  LEN/LENB, MID/MIDB, REPLACE/REPLACEB, RIGHT/RIGHTB, SEARCH/SEARCHB), BETA.INV, and COPILOT and
  PY (Microsoft 365 functions not on that page when fetched).
- **Ours (505):** every function spec in `crates/functions/src/*.rs` (`f!("NAME", …)`) plus the
- **Ours (503):** every function spec in `crates/functions/src/*.rs` (`f!("NAME", …)`) plus the
- **Ours (502):** every function spec in `crates/functions/src/*.rs` (`f!("NAME", …)`) plus the
- **Ours (502):** every function spec in `crates/functions/src/*.rs` (`f!("NAME", …)`) plus the
- **Ours (505):** every function spec in `crates/functions/src/*.rs` (`f!("NAME", …)`) plus the
  evaluator's special forms in `crates/calc/src/eval.rs` (`SPECIAL_FUNCTIONS`: IF, IFS, CHOOSE,
  OFFSET, INDIRECT, INDEX, SUBTOTAL, AGGREGATE, TEXT, LET, LAMBDA, MAP, REDUCE, SCAN, BYROW, BYCOL,
  MAKEARRAY…), excluding the test-only `ADD2`. Every one is an Excel name.
- **Working** excludes stubs that return an error by design: the 7 CUBE functions (`#N/A`, no
  cube connections) and WEBSERVICE and FILTERXML (`#VALUE!`). Excel for Mac also lacks
  WEBSERVICE and FILTERXML; Excel for Windows has them.
- The measurement is **presence**. Depth (edge cases matching Excel) is estimated below.

## Coverage

**Measured: 513 / 523 by name (98.1%); 504 / 523 working (96.4%).**

| Category | Excel | Ours | Working | % working | Missing |
|---|---|---|---|---|---|
| Statistical | 111 | 111 | 111 | 100% | — |
| Math and trigonometry | 80 | 79 | 79 | 99% | PERCENTOF |
| Financial | 55 | 55 | 55 | 100% | — |
| Engineering | 54 | 54 | 54 | 100% | — |
| Text | 50 | 48 | 48 | 96% | DETECTLANGUAGE, TRANSLATE |
| Lookup and reference | 40 | 36 | 36 | 90% | GETPIVOTDATA, IMAGE, RTD, TRIMRANGE |
| Compatibility | 40 | 40 | 40 | 100% | — |
| Date and time | 25 | 25 | 25 | 100% | — |
| Information | 22 | 21 | 21 | 95% | STOCKHISTORY |
| Logical | 19 | 19 | 19 | 100% | — |
| Database | 12 | 12 | 12 | 100% | — |
| Cube | 7 | 7 | 0 | 0% | (all 7 are `#N/A` stubs) |
| Web | 3 | 3 | 1 | 33% | (WEBSERVICE, FILTERXML are stubs; ENCODEURL works) |
| Add-in and Automation | 3 | 1 | 1 | 33% | CALL, REGISTER.ID |
| AI and Python | 2 | 0 | 0 | 0% | COPILOT, PY |
| **Total** | **523** | **505** | **496** | **94.8%** | 18 missing, 9 stubs |

By the Excel version that introduced them (working / total): before 2010 346 / 360, 2010
56 / 56, 2013 49 / 51, 2016 1 / 5, 2019 6 / 6, 2021 11 / 11, 2024 22 / 23, Microsoft 365 5 / 11.
Dynamic-array and LAMBDA-era functions (FILTER, SORT, UNIQUE, XLOOKUP, VSTACK, TAKE, GROUPBY,
PIVOTBY, REGEXTEST…) are all present.

### Scope of what's missing

| Function(s) | In scope? | Estimate | Note |
|---|---|---|---|
| FORECAST.ETS family (4) | yes | 4–6 h | AAA exponential smoothing with seasonality detection; needed for Forecast Sheet too |
| ODDFPRICE, ODDFYIELD, ODDLPRICE, ODDLYIELD | yes | 2–4 h | Odd first/last period bond maths |
| GETPIVOTDATA | yes | 2–3 h | Needs the pivot engine's result cache; also the "Generate GetPivotData" option |
| TRIMRANGE, BAHTTEXT | yes | 1–2 h | TRIMRANGE also implies the `.:` trim-ref operators |
| TRIMRANGE, PERCENTOF | yes | 1–2 h | TRIMRANGE also implies the `.:` trim-ref operators |
| PERCENTOF, BAHTTEXT | yes | 1–2 h | |
| Trim references (`A1.:.E10`, `A1:.E10`, `A1.:E10`) | yes | 1–2 h | TRIMRANGE works; the operators need lexer support (`.` is a name character) |
| IMAGE | yes | 3–5 h | Pictures in cells (see PR #105); fetching URLs needs a privacy decision |
| CUBE* (7), WEBSERVICE, FILTERXML | partly | 4–8 h | FILTERXML can work offline; CUBE needs an OLAP/data-model source |
| TRANSLATE, DETECTLANGUAGE, STOCKHISTORY, COPILOT, PY | no (cloud) | — | Microsoft cloud services; open equivalents only by owner decision |
| CALL, REGISTER.ID, RTD | no | — | Native DLL/COM calls; not portable or safe |

## Formula language

| Feature | Status |
|---|---|
| A1 and R1C1 references, absolute/relative/mixed, F4 cycling | done |
| 3-D references (`Sheet1:Sheet3!A1`) | done; endpoint-deletion bug (#140) |
| Structured references (`Table1[Col]`, `[#Totals]`, `@`) | done |
| Defined names (workbook and sheet scope), Create from Selection | done |
| Dynamic arrays, spill, `#SPILL!`, spill operator `A1#` | done; stale-spill bugs (#133, #139; PRs #148, #157) |
| Implicit intersection `@`, legacy CSE array formulas | done; legacy array range fill (PR #156) |
| LET, LAMBDA, named LAMBDAs, ISOMITTED, MAP/REDUCE/SCAN/BYROW/BYCOL/MAKEARRAY | done |
| Iterative calculation for circular references | cycle detection done; the Enable Iterative Calculation setting is stored and saved to XLSX, but the calc engine does not iterate (circular references stay unresolved) |
| 15-significant-digit comparison semantics | **missing** (#57: `=0.1+0.2=0.3` is FALSE) |
| External workbook references (`[Book.xlsx]Sheet1!A1`) | parsed; links dropped on open, no update |
| Localized function names and `;` separators | **missing** (#45); en-US only |
| Error values | the seven classic errors plus `#SPILL!`, `#CALC!`, `#GETTING_DATA` done; `#FIELD!`, `#BLOCKED!`, `#CONNECT!`, `#BUSY!`, `#UNKNOWN!`, `#PYTHON!` missing (minor: they come from data types, cloud and Python features) |

## Depth (estimated)

**~82% ready.** The library is broad and has 96 test functions in `crates/functions` with many
assertions each, but there is no conformance suite against Excel-computed results, and user
reports keep finding edge cases: MIN() with no arguments (#138), COUNTBLANK past the used range
(#137), ROUND at 15 digits and DATE(1900,2,29) (fixed 10-09), SUMIF sum-range sizing (fixed
10-09). Statistical distributions and financial functions with iterative solvers (RATE, IRR,
XIRR, YIELD) are the highest-risk for last-digit differences.

Work to close (16–26 h in total): the in-scope missing functions (10–16 h with the table above),
an oracle corpus of Excel-computed results per function (owner runs Excel on generated
workbooks; results checked in as data, not Excel files) and fixing what it finds (6–10 h).

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-11 | minor | ODDFPRICE, ODDFYIELD, ODDLPRICE, ODDLYIELD added: Financial 55 / 55; 505 / 523 by name, 496 working |
| 2026-10-11 | minor | PERCENTOF and EUROCONVERT added: 503 / 523 by name, 494 working |
| 2026-10-11 | minor | BAHTTEXT added: 502 / 523 by name, 493 working |
| 2026-10-11 | minor | TRIMRANGE added (the trim-reference operators not yet): 502 / 523 by name, 493 working |
| 2026-10-11 | minor | FORECAST.ETS, .CONFINT, .SEASONALITY, .STAT added (AAA exponential smoothing; Excel's optimiser is undocumented, so last digits on noisy data may differ): 505 / 523 by name, 496 working |
| 2026-10-10 | major | Created: 501 / 523 by name, 492 working, per category, against Microsoft's published list |
