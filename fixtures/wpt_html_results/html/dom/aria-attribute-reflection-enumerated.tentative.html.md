# html/dom/aria-attribute-reflection-enumerated.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/aria-attribute-reflection-enumerated.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML5 reflection tests: ARIA enumerated attributes</title>
</head>

<body>
<!-- tentative: https://github.com/w3c/aria/pull/2484 -->
<!-- ARIA attributes which were converted from DOMString to enumerated:
    - aria-atomic
    - aria-autocomplete
    - aria-busy
    - aria-checked
    - aria-current
    - aria-disabled
    - aria-expanded
    - aria-haspopup
    - aria-hidden
    - aria-invalid
    - aria-live
    - aria-modal
    - aria-multiline
    - aria-multiselectable
    - aria-orientation
    - aria-pressed
    - aria-readonly
    - aria-required
    - aria-selected
    - aria-sort
-->
<div id="log"></div>
<script src="/resources/testharness.js"></script>
<script src=/resources/testharnessreport.js></script>
<script src=original-harness.js></script>
<script src=new-harness.js></script>
<script src=elements-aria-enumerated.js></script>
<script src=reflection.js></script>
</body>

</html>

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
  "source_name": "html/dom/aria-attribute-reflection-enumerated.tentative.html"
}
```
