# html/browsers/windows/browsing-context-names/choose-_parent-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-_parent-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Choose browsing context - '_parent' (nested contexts)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(t => {
  var topWindow;
  t.add_cleanup(() => topWindow.close());
  window.addEventListener('message', t.step_func_done(e => {
    assert_equals(e.data.name, 'iframeParent');
    assert_false(e.data.isTop, 'window.parent is not top');
  }));
  topWindow = window.open('resources/choose-_parent-002-window.html', '_blank');
}, 'choosing _parent context: multiple nested contexts');
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-_parent-002.html"
}
```
