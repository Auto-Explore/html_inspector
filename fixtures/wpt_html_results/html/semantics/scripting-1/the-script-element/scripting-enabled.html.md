# html/semantics/scripting-1/the-script-element/scripting-enabled.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/scripting-enabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>JS is disabled on documents created without a browsing context</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#concept-n-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function(t) {
  var doc = document.implementation.createHTMLDocument();
  window.fail_test = t.unreached_func('should not have been called');

  var script = doc.createElement('script');
  script.textContent = 'fail_test();';
  doc.documentElement.appendChild(script);
}, 'script on document returned by createHTMLDocument should not execute');
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
  "source_name": "html/semantics/scripting-1/the-script-element/scripting-enabled.html"
}
```
