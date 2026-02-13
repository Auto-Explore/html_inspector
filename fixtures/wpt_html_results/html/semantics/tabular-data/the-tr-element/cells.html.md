# html/semantics/tabular-data/the-tr-element/cells.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-tr-element/cells.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLTableRowElement#cells</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<table>
  <tr id="testTr">
    <td>First</td>
    <div><td>Second</td></div>
    <td>Third
      <table>
        <tr><td>Nested first</td></tr>
      </table>
    </td>
    <img>
  </tr>
</table>
<script>
var tr = document.getElementById("testTr");

test(function () {
  tr.insertBefore(document.createElementNS("foo", "td"), tr.children[1]);
  assert_array_equals(tr.cells, [tr.children[0], tr.children[2], tr.children[3]]);
}, "HTMLTableRowElement cells ignores nested tables and non-HTML elements");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 387,
        "byte_start": 382,
        "col": 5,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 387,
        "byte_start": 382,
        "col": 5,
        "line": 18
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
  "source_name": "html/semantics/tabular-data/the-tr-element/cells.html"
}
```
