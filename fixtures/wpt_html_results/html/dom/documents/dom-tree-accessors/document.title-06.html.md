# html/dom/documents/dom-tree-accessors/document.title-06.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.title-06.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.title and the empty string</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#document.title">
<meta name="assert" content="On setting document.title to the empty string, no text node must be created.">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var head = document.documentElement.firstChild;
  head.removeChild(head.firstChild);
  assert_equals(document.title, "");
  document.title = "";
  assert_equals(document.title, "");
  assert_true(head.lastChild instanceof HTMLTitleElement, "Need a title element.");
  assert_equals(head.lastChild.firstChild, null);
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.title-06.html"
}
```
