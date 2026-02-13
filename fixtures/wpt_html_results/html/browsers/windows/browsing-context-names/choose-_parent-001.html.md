# html/browsers/windows/browsing-context-names/choose-_parent-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-_parent-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Choose browsing context - '_parent'</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(t => {
  window.addEventListener('message', t.step_func_done(e => {
    assert_equals(e.data.name, 'parentWin');
  }));
}, 'The parent browsing context must be chosen if the given name is `_parent`');
</script>
<iframe id="embedded" src="resources/choose-_parent-001-iframe-1.html" name="parentWin" style="display:none"></iframe>
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-_parent-001.html"
}
```
