# html/browsers/the-window-object/open-close/open-features-tokenization-noopener.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/open-features-tokenization-noopener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML: window.open `features`: tokenization -- `noopener`</title>
<meta name=timeout content=long>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#apis-for-creating-and-navigating-browsing-contexts-by-name">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/tokenization-noopener-noreferrer.js"></script>
<script>
  booleanTests("noopener");
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
  "source_name": "html/browsers/the-window-object/open-close/open-features-tokenization-noopener.html"
}
```
