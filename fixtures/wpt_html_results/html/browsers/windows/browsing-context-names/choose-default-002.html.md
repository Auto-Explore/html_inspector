# html/browsers/windows/browsing-context-names/choose-default-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-default-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Browsing context names - empty string</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(t => {
  window.addEventListener('message', t.step_func_done(e => {
    assert_equals(e.data.isTop, false);
    assert_equals(e.data.name, 'hellothere', 'Empty-string browsing context should choose current context');
  }), false);
}, 'The current browsing context must be chosen if the given name is empty string');
</script>
<iframe name="hellothere" src="resources/choose-default-002-iframe.html"></iframe>
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-default-002.html"
}
```
