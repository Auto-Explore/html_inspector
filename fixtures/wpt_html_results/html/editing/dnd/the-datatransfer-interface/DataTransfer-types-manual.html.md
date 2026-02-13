# html/editing/dnd/the-datatransfer-interface/DataTransfer-types-manual.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-datatransfer-interface/DataTransfer-types-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>DataTransferItem Test: types - files</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/dnd.html#dom-datatransfer-types"/>
<meta name="flags" content="interact">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<p><div id="div" style="border: 2px green solid; width: 200px; height: 200px;"></div></p>

<h2>Test steps:</h2>
<p>Drag a file enter the green box, then drop file out</p>

<script>

let div = document.getElementById("div");

setup({explicit_done: true});
setup({explicit_timeout: true});

on_event(div, "dragenter", evt => {
  let type = evt.dataTransfer.types[0];
  test(() => {
    assert_equals(type, "Files");
  }, "Check if one of the types will be the string 'Files' when drag a file");
  done();
});

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.no_p_in_scope",
      "message": "No “p” element in scope but a “p” end tag seen.",
      "severity": "Error",
      "span": {
        "byte_end": 485,
        "byte_start": 481,
        "col": 86,
        "line": 10
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
  "source_name": "html/editing/dnd/the-datatransfer-interface/DataTransfer-types-manual.html"
}
```
