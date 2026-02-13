# html/semantics/tabular-data/processing-model-1/span-limits.html

Counts:
- errors: 0
- warnings: 11
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/processing-model-1/span-limits.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Limits on colSpan/rowSpan</title>
<meta name="timeout" content="long">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>

<table border=1>
  <tr><td colspan=500>a<td colspan=500 id=a1>a
  <!-- This cell must span the previous two -->
  <tr><td colspan=1000 id=a2>a
</table>

<table border=1>
  <tr><td colspan=1000 id=b1>a<td>a
  <!-- This cell must span only the first cell in the previous row -->
  <tr><td colspan=1001 id=b2>a
</table>

<table border=1 style="float:left">
  <!-- The first column must go all the way down to the bottom -->
  <tr><td rowspan=65534 id=c1>a<td>
  <!-- We'll add another 65533 rows later -->
</table>

<table border=1>
  <!-- The first column must go one cell below the bottom -->
  <tr><td rowspan=65535 id=d1>a<td>
  <!-- We'll add another 65534 rows later -->
</table>

<table>
    <tr>
        <td id="rowspan-limit-test1" rowspan=5></td>
        <td id="rowspan-limit-test2" rowspan=0></td>
        <td id="rowspan-limit-test3" rowspan=1000></td>
        <td id="rowspan-limit-test4" rowspan=65534></td>
        <td id="rowspan-limit-test5" rowspan=65535></td>
        <td id="rowspan-limit-test6" rowspan=5555555></td>
    </tr>
</table>

<script>
var $ = document.querySelector.bind(document);

test(() => {
    assert_equals($("#a2").getBoundingClientRect().right,
                  $("#a1").getBoundingClientRect().right);
}, "colspan of 1000 must work");

test(() => {
    assert_equals($("#b2").getBoundingClientRect().right,
                  $("#b1").getBoundingClientRect().right);
}, "colspan of 1001 must be treated as 1000");

test(() => {
    var s = "";
    for (var i = 0; i < 65532; i++) {
      s += "<tr><td>";
    }
    s += "<tr><td id=c2>";
    document.querySelectorAll("table")[2].firstElementChild.innerHTML += s;
    assert_equals($("#c1").getBoundingClientRect().bottom,
                  $("#c2").getBoundingClientRect().bottom);
}, "rowspan of 65534 must work");

test(() => {
    var s = "";
    for (var i = 0; i < 65532; i++) {
      s += "<tr><td>";
    }
    s += "<tr><td id=d2><tr><td>a<td>";
    document.querySelectorAll("table")[3].firstElementChild.innerHTML += s;
    assert_equals($("#d1").getBoundingClientRect().bottom,
                  $("#d2").getBoundingClientRect().bottom);
}, "rowspan of 65535 must be treated as 65534");

test(() => {
    let td = document.createElement("td");
    td.rowSpan = 5;
    assert_equals(td.rowSpan, 5);

    td.rowSpan = 0;
    assert_equals(td.rowSpan, 0);

    td.rowSpan = 1000;
    assert_equals(td.rowSpan, 1000);

    td.rowSpan = 65534;
    assert_equals(td.rowSpan, 65534);

    td.rowSpan = 65535;
    assert_equals(td.rowSpan, 65534);

    td.rowSpan = 555555;
    assert_equals(td.rowSpan, 65534);
}, "rowspan must be clamped to [0, 65534] when set via script");

test(() => {
    assert_equals(document.getElementById("rowspan-limit-test1").rowSpan, 5);
    assert_equals(document.getElementById("rowspan-limit-test2").rowSpan, 0);
    assert_equals(document.getElementById("rowspan-limit-test3").rowSpan, 1000);
    assert_equals(document.getElementById("rowspan-limit-test4").rowSpan, 65534);
    assert_equals(document.getElementById("rowspan-limit-test5").rowSpan, 65534);
    assert_equals(document.getElementById("rowspan-limit-test6").rowSpan, 65534);
}, "rowspan must be clamped to [0, 65534] when parsing attributes");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.table.columns.no_starting_cells",
      "message": "Table columns in range 2…500 established by element “td” have no cells beginning in them.",
      "severity": "Warning",
      "span": {
        "byte_end": 367,
        "byte_start": 359,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.colspan.max",
      "message": "The value of the “colspan” attribute must be less than or equal to 1000.",
      "severity": "Warning",
      "span": {
        "byte_end": 522,
        "byte_start": 499,
        "col": 7,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.table.columns.no_starting_cells",
      "message": "Table columns in range 2…1000 established by element “td” have no cells beginning in them.",
      "severity": "Warning",
      "span": {
        "byte_end": 532,
        "byte_start": 524,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.less_than_first_row",
      "message": "A table row was 1000 columns wide, which is less than the column count established by the first row (1001).",
      "severity": "Warning",
      "span": {
        "byte_end": 532,
        "byte_start": 524,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.spans_past_row_group",
      "message": "Table cell spans past the end of its row group established by a “tbody” element; clipped to the end of the row group.",
      "severity": "Warning",
      "span": {
        "byte_end": 727,
        "byte_start": 719,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.rowspan.max",
      "message": "The value of the “rowspan” attribute must be less than or equal to 65534.",
      "severity": "Warning",
      "span": {
        "byte_end": 838,
        "byte_start": 814,
        "col": 7,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.spans_past_row_group",
      "message": "Table cell spans past the end of its row group established by a “tbody” element; clipped to the end of the row group.",
      "severity": "Warning",
      "span": {
        "byte_end": 898,
        "byte_start": 890,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.rowspan.max",
      "message": "The value of the “rowspan” attribute must be less than or equal to 65534.",
      "severity": "Warning",
      "span": {
        "byte_end": 1187,
        "byte_start": 1144,
        "col": 9,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.rowspan.max",
      "message": "The value of the “rowspan” attribute must be less than or equal to 65534.",
      "severity": "Warning",
      "span": {
        "byte_end": 1246,
        "byte_start": 1201,
        "col": 9,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.table.cell.spans_past_row_group",
      "message": "Table cell spans past the end of its row group established by a “tbody” element; clipped to the end of the row group.",
      "severity": "Warning",
      "span": {
        "byte_end": 1270,
        "byte_start": 1262,
        "col": 1,
        "line": 41
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
  "source_name": "html/semantics/tabular-data/processing-model-1/span-limits.html"
}
```
