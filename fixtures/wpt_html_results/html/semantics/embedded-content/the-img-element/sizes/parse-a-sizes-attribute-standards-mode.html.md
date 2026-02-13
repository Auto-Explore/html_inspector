# html/semantics/embedded-content/the-img-element/sizes/parse-a-sizes-attribute-standards-mode.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/sizes/parse-a-sizes-attribute-standards-mode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img parse a sizes attribute (standards mode)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<iframe data-desc="standards mode" style="width:1000px; height:1000px" src="support/sizes-iframed.sub.html?doctype=doctype%20html&style="></iframe>
<script src="support/parse-a-sizes-attribute.js"></script>
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
  "source_name": "html/semantics/embedded-content/the-img-element/sizes/parse-a-sizes-attribute-standards-mode.html"
}
```
