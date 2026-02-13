# html/dom/elements/global-attributes/id-name.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/id-name.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>id and name attributes and getElementById</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-id-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<div name="abcd"></div>
<p name="abcd" id="abcd"></p>
</div>
<script>
test(function() {
  assert_equals(document.getElementById("abcd").nodeName, "P");
  assert_equals(document.getElementById("abcd").localName, "p");
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.div.name.disallowed",
      "message": "Attribute “name” not allowed on element “div” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 381,
        "byte_start": 364,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/elements/global-attributes/id-name.html"
}
```
