# html/semantics/document-metadata/the-title-element/title.text-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-title-element/title.text-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>title.text with comment and element children.</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-title-text">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
try {
  var title = document.getElementsByTagName("title")[0];
  while (title.childNodes.length)
    title.removeChild(title.childNodes[0]);
  title.appendChild(document.createComment("COMMENT"));
  title.appendChild(document.createTextNode("TEXT"));
  title.appendChild(document.createElement("a"))
       .appendChild(document.createTextNode("ELEMENT"))
} catch (e) {
}
</script>
<script>
test(function() {
  assert_equals(title.text, "TEXT");
  assert_equals(title.textContent, "TEXTELEMENT");
})
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
  "source_name": "html/semantics/document-metadata/the-title-element/title.text-01.html"
}
```
