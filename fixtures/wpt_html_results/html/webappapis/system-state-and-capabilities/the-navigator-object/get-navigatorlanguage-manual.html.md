# html/webappapis/system-state-and-capabilities/the-navigator-object/get-navigatorlanguage-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/get-navigatorlanguage-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>NavigatorLanguage: navigator.language returns the user's preferred language</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#navigatorlanguage">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<h2>Precondition</h2>
<p>The user agent's preferred language is set as English (en).</p>
<div id="log"></div>
<script>
  test(function() {
    assert_equals(navigator.language, "en");
  });
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
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/get-navigatorlanguage-manual.html"
}
```
