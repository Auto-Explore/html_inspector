# html/webappapis/system-state-and-capabilities/the-navigator-object/navigator-indexed.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigator-indexed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test for lack of indexed getter on Navigator</title>
<link rel="author" title="Ms2ger" href="mailto:Ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-navigator-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  assert_false("0" in window.navigator);
  assert_equals(window.navigator[0], undefined);
}, "window.navigator[0] should not exist");
test(function() {
  window.navigator[0] = "pass";
  assert_true("0" in window.navigator);
  assert_equals(window.navigator[0], "pass");
}, "window.navigator[0] should be settable");
test(function() {
  assert_false("-1" in window.navigator);
  assert_equals(window.navigator[-1], undefined);
}, "window.navigator[-1] should not exist");
test(function() {
  window.navigator[-1] = "pass";
  assert_true("-1" in window.navigator);
  assert_equals(window.navigator[-1], "pass");
}, "window.navigator[-1] should be settable");
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
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigator-indexed.html"
}
```
