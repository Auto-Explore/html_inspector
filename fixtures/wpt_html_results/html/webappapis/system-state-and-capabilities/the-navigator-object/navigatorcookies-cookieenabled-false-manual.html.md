# html/webappapis/system-state-and-capabilities/the-navigator-object/navigatorcookies-cookieenabled-false-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigatorcookies-cookieenabled-false-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>navigator.cookieEnabled false</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/#dom-navigator-cookieenabled">
<meta name="flags" content="interact">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<h2>Preconditions</h2>
<p>Disable cookies in browser settings.</p>

<script>
  test(() => {
    assert_false(navigator.cookieEnabled);
  }, "navigator.cookieEnabled is false when cookies are disabled");
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
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigatorcookies-cookieenabled-false-manual.html"
}
```
