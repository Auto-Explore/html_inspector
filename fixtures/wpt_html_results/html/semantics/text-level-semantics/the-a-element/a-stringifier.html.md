# html/semantics/text-level-semantics/the-a-element/a-stringifier.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/a-stringifier.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLAnchorElement stringifier</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://webidl.spec.whatwg.org/#es-stringifier">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/stringifiers.js></script>
<div id=log></div>
<script>
test(function() {
  test_stringifier_attribute(document.createElement("a"), "href", false);
  var a = document.createElement("a");
  a.setAttribute("href", "foo");
  test_stringifier_attribute(a, "href", false);
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
  "source_name": "html/semantics/text-level-semantics/the-a-element/a-stringifier.html"
}
```
