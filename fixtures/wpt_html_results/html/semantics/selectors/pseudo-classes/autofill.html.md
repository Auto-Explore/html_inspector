# html/semantics/selectors/pseudo-classes/autofill.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/autofill.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:autofill)</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/css/support/parsing-testcommon.js"></script>
<script>
test_valid_selector(":autofill");
test_valid_selector(":-webkit-autofill", [":autofill", ":-webkit-autofill"]);
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
  "source_name": "html/semantics/selectors/pseudo-classes/autofill.html"
}
```
