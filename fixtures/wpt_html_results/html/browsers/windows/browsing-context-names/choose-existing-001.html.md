# html/browsers/windows/browsing-context-names/choose-existing-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-existing-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Choose browsing context - the given name is same as an existing browsing context's name</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<iframe src="resources/choose-existing-001-iframe.html" style="display:none"></iframe>
<iframe name="iExist" style="display:none"></iframe>
<script>
async_test(t => {
  window.addEventListener('message', t.step_func_done(e => {
    assert_equals(e.data.name, 'iExist');
  }), false);

}, 'An existing browsing context must be chosen if the given name is the same as its name');
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-existing-001.html"
}
```
