# html/browsers/origin/cross-origin-objects/cross-origin-objects-on-new-window.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/cross-origin-objects/cross-origin-objects-on-new-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name="timeout" content="long">
<title>Cross-origin behavior of Window and Location on new Window</title>
<link rel="author" title="Bobby Holley (:bholley)" href="bobbyholley@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#security-window">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#security-location">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
setup({explicit_done: true});

window.addEventListener('message', function onmessage(evt) {
  window.removeEventListener('message', onmessage);
  test(function() {
    var results = evt.data;
    assert_true(results.length > 0, 'Need results');
    results.forEach(function(r) { assert_true(r.pass, r.message); });
  }, "Cross-origin object identity preserved across document.domain");
  win.close();
  done();
});
var win = window.open('win-documentdomain.sub.html');
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
  "source_name": "html/browsers/origin/cross-origin-objects/cross-origin-objects-on-new-window.html"
}
```
