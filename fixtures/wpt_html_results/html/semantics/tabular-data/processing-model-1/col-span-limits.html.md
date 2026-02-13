# html/semantics/tabular-data/processing-model-1/col-span-limits.html

Counts:
- errors: 0
- warnings: 11
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/processing-model-1/col-span-limits.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Limits on col/colgroup.span</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
  div.square {
    height:20px;
    width:20px;
    border:1px solid lime;
  }
  main table {
    border-collapse:collapse;
    border:1px solid blue;
  }
  main table col {
    border-left:2px solid black;
  }
</style>
<div id=log></div>
<main>
<table id=table1>
  <col span=1000>
  <tr>
    <td colspan=999><div class="square"></div></td>
    <td><div class="square"></div></td>
  </tr>
  <tr>
    <td colspan=1000><div class="square"></div></td>
  </tr>
</table>
<br>
These two must look the same, each having 2 cells in one row:
<table id=table2>
  <col span=1000>
  <tr>
    <td colspan=1000><div class="square"></div></td>
    <td><div class="square"></div></td>
  </tr>
</table>
<br>
<table id=table3>
  <col id="colspan-3" span=1001>
  <tr>
    <td colspan=1000><div class="square"></div></td>
    <td><div class="square"></div></td>
  </tr>
</table>
<table>
    <tr>
        <td id="colspan-limit-test1" colspan=5></td>
        <td id="colspan-limit-test2" colspan=0></td>
        <td id="colspan-limit-test3" colspan=1000></td>
        <td id="colspan-limit-test4" colspan=1001></td>
        <td id="colspan-limit-test5" colspan=5555555></td>
    </tr>
</table>
</main>

<script>
test(() => {
    assert_equals(table1.offsetWidth, 53);
}, "col span of 1000 must work");

test(() => {
    assert_equals(table2.offsetWidth, 51, "table2 width");
    assert_equals(table3.offsetWidth, 51, "table3 width");
}, "col span of 1001 must be treated as 1000");

test(() => {
    let td = document.createElement("td");
    td.colSpan = 5;
    assert_equals(td.colSpan, 5);

    td.colSpan = 0;
    assert_equals(td.colSpan, 1);

    td.colSpan = 1000;
    assert_equals(td.colSpan, 1000);

    td.colSpan = 1001;
    assert_equals(td.colSpan, 1000);

    td.colSpan = 555555;
    assert_equals(td.colSpan, 1000);
}, "colspan must be clamped to [1, 1000] when set via script");

test(() => {
    assert_equals(document.getElementById("colspan-limit-test1").colSpan, 5);
    assert_equals(document.getElementById("colspan-limit-test2").colSpan, 1);
    assert_equals(document.getElementById("colspan-limit-test3").colSpan, 1000);
    assert_equals(document.getElementById("colspan-limit-test4").colSpan, 1000);
    assert_equals(document.getElementById("colspan-limit-test5").colSpan, 1000);
}, "colspan must be clamped to [1, 1000] when parsing attributes");

test(() => {
    let column = document.getElementById("colspan-3");
    column.span = 5;
    assert_equals(column.span, 5);

    column.span = 0;
    assert_equals(column.span, 1);

    column.span = 1000;
    assert_equals(column.span, 1000);

    column.span = 1001;
    assert_equals(column.span, 1000);

    column.span = 555555;
    assert_equals(column.span, 1000);
}, "column span must be clamped to [1, 1000] when set via script");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.table.columns.no_starting_cells",
      "message": "Table columns in range 2…999 established by element “td” have no cells beginning in them.",
      "severity": "Warning",
      "span": {
        "byte_end": 634,
        "byte_start": 626,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.exceeds_col_markup",
      "message": "A table row was 1001 columns wide and exceeded the column count established using column markup (1000).",
      "severity": "Warning",
      "span": {
        "byte_end": 854,
        "byte_start": 846,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.table.columns.no_starting_cells",
      "message": "Table columns in range 2…1000 established by element “td” have no cells beginning in them.",
      "severity": "Warning",
      "span": {
        "byte_end": 854,
        "byte_start": 846,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.table.col.span.max",
      "message": "The value of the “span” attribute must be less than or equal to 1000.",
      "severity": "Warning",
      "span": {
        "byte_end": 910,
        "byte_start": 880,
        "col": 3,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.exceeds_col_markup",
      "message": "A table row was 1001 columns wide and exceeded the column count established using column markup (1000).",
      "severity": "Warning",
      "span": {
        "byte_end": 1027,
        "byte_start": 1019,
        "col": 1,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.table.columns.no_starting_cells",
      "message": "Table columns in range 2…1000 established by element “td” have no cells beginning in them.",
      "severity": "Warning",
      "span": {
        "byte_end": 1027,
        "byte_start": 1019,
        "col": 1,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.colspan.zero",
      "message": "Bad value “0” for attribute “colspan” on element “td”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1145,
        "byte_start": 1106,
        "col": 9,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.colspan.max",
      "message": "The value of the “colspan” attribute must be less than or equal to 1000.",
      "severity": "Warning",
      "span": {
        "byte_end": 1257,
        "byte_start": 1215,
        "col": 9,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.colspan.max",
      "message": "The value of the “colspan” attribute must be less than or equal to 1000.",
      "severity": "Warning",
      "span": {
        "byte_end": 1316,
        "byte_start": 1271,
        "col": 9,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.table.columns.no_starting_cells",
      "message": "Table columns in range 2…5 established by element “td” have no cells beginning in them.",
      "severity": "Warning",
      "span": {
        "byte_end": 1340,
        "byte_start": 1332,
        "col": 1,
        "line": 56
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/tabular-data/processing-model-1/col-span-limits.html"
}
```
