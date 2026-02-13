# html/browsers/windows/browsing-context-names/choose-default-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-default-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Browsing context - Default name</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<iframe src="/common/blank.html" style="display:none"></iframe>
<object id="obj" type="text/html" data="about:blank"></object>
<embed id="embedded" type="image/svg+xml" src="/images/green.svg" width="0" height="0" />
<script>
test(t => {
  assert_equals(window.frames[0].name, "");
  assert_equals(document.getElementById("embedded").name, "");
  assert_equals(window["obj"].name, "");
}, "A embedded browsing context has empty-string default name");

test(t => {
  var win = window.open("about:blank", "_blank");
  assert_equals(win.name, "");
  win.close();
}, "A browsing context which is opened by window.open() method with '_blank' parameter has empty-string default name");
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-default-001.html"
}
```
