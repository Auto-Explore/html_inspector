# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-grid-flex-multicol.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-grid-flex-multicol.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend and flexbox, grid & multicol</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
legend { width: 200px; background: silver }
#flex { display: flex; }
#inline-flex { display: inline-flex; }
#grid { display: grid; }
#inline-grid { display: inline-grid; }
#grid, #inline-grid { grid-template-columns: auto auto }
#multicol { columns: 2; }
</style>
<fieldset><legend id=ref>12</legend></fieldset>
<fieldset><legend id=flex><div>1</div><div>2</div></legend></fieldset>
<fieldset><legend id=inline-flex><div>1</div><div>2</div></legend></fieldset>
<fieldset><legend id=grid><div>1</div><div>2</div></legend></fieldset>
<fieldset><legend id=inline-grid><div>1</div><div>2</div></legend></fieldset>
<fieldset><legend id=multicol><div>1</div><div>2</div></legend></fieldset>
<script>
 const ref = document.getElementById('ref');
 for (const id of ["flex", "inline-flex", "grid", "inline-grid", "multicol"]) {
   test(() => {
     const elm = document.getElementById(id);
     assert_equals(elm.offsetHeight, ref.offsetHeight, 'offsetHeight');
     if (id !== "multicol") {
       assert_equals(getComputedStyle(elm).display, id, 'display');
     }
   }, id);
 }
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-grid-flex-multicol.html"
}
```
