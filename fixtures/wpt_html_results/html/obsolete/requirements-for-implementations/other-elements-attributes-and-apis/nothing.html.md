# html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/nothing.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/nothing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Methods that must do nothing: clear(), captureEvents(), and releaseEvents()</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function() {
  assert_equals(document.clear(), undefined);
}, "document.clear");

test(function() {
  assert_equals(document.captureEvents(), undefined);
}, "document.captureEvents");

test(function() {
  assert_equals(document.releaseEvents(), undefined);
}, "document.releaseEvents");

test(function() {
  assert_equals(window.captureEvents(), undefined);
}, "window.captureEvents");

test(function() {
  assert_equals(window.releaseEvents(), undefined);
}, "window.releaseEvents");
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
  "source_name": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/nothing.html"
}
```
