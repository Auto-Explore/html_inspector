# html/webappapis/system-state-and-capabilities/the-navigator-object/navigatorcookies-cookieenabled-true.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigatorcookies-cookieenabled-true.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>navigator.cookieEnabled true</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/#dom-navigator-cookieenabled">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
  test(() => {
    assert_true(navigator.cookieEnabled);
  }, "navigator.cookieEnabled is true when cookies are enabled");
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
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigatorcookies-cookieenabled-true.html"
}
```
