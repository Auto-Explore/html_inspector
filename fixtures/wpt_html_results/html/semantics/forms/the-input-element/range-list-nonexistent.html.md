# html/semantics/forms/the-input-element/range-list-nonexistent.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-list-nonexistent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class=reftest-wait>
<title>The range is repainted if the ID first identifies no list, then a list takes on that ID</title>
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1805105">
<link rel=author href="mailto:zach@zrhoffman.net" title="Zach Hoffman">
<link rel=match href=range-tick-marks-05-ref.html>
<script src=/common/reftest-wait.js></script>
<input type=range step=3 value=1 min=-5 max=5 list=nonexistentlist>
<datalist>
  <option value=4>
  <option value=-2>
</datalist>
<script>
  requestAnimationFrame(() =>
    requestAnimationFrame(() => {
      const dataListWithIDOfNonExistentList = document.querySelector("datalist");
      dataListWithIDOfNonExistentList.id = "nonexistentlist";
      takeScreenshot();
    }));
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.list.must_refer_to_datalist",
      "message": "The “list” attribute of the “input” element must refer to a “datalist” element.",
      "severity": "Warning",
      "span": {
        "byte_end": 457,
        "byte_start": 390,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/forms/the-input-element/range-list-nonexistent.html"
}
```
