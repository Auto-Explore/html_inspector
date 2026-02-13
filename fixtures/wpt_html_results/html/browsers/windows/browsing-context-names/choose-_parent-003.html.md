# html/browsers/windows/browsing-context-names/choose-_parent-003.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-_parent-003.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Choose browsing context - '_parent' (via window.open)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(t => {
  var topWindow;
  t.add_cleanup(() => topWindow.close());
  window.addEventListener('message', t.step_func_done(e => {
    assert_equals(e.data.name, 'parentTopReplace');
    assert_equals(e.data.isTop, true);
  }));
  topWindow = window.open('resources/choose-_parent-003-window.html', 'parentTopReplace');
}, '_parent should reuse window.parent context');
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-_parent-003.html"
}
```
