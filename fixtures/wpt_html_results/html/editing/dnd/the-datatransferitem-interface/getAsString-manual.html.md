# html/editing/dnd/the-datatransferitem-interface/getAsString-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-datatransferitem-interface/getAsString-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>DataTransferItem Test: getAsString()</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<p><input type="text" value="dragcharacters" style="border:2px blue solid; width:200px; height: 100px;"/></p>
<p><input id="container" type="text" style="border:2px green solid; width:200px; height: 100px;"/></p>

<p>Select all characters in blue box and drag to green box then drop on the green box</p>

<script>

setup({explicit_done : true, explicit_timeout : true});

let container = document.getElementById("container");

on_event(container, "drop", evt => {
  let item = evt.dataTransfer.items[0];

  test(() => {
    let file1 = item.getAsFile();
    assert_equals(file1, null);
  }, "Check if DataTransferItem.getAsFile return null if drag data item kind is not File");

  let data;
  item.getAsString(str => {
    data = str;
  });
  setTimeout(() => {
    test(() => {
      assert_equals(data, "dragcharacters");
    }, "Check if DataTransferItem.getAsString return the dragged string");
    done();
  }, 0);
});

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
  "source_name": "html/editing/dnd/the-datatransferitem-interface/getAsString-manual.html"
}
```
