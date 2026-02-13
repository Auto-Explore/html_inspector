# html/editing/dnd/historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Historical drag-and-drop features</title>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

test(() => {
  const potentialBadLocations = [
    window,
    document,
    HTMLElement.prototype,
    SVGElement.prototype,
    Document.prototype,
    HTMLDocument.prototype,
    Element.prototype
  ];
  for (const location of potentialBadLocations) {
    assert_false("ondragexit" in location,
      `${location.constructor.name} must not have a property "ondragexit"`);
  }
}, `ondragexit must not be present on the GlobalEventHandlers locations`);
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
  "source_name": "html/editing/dnd/historical.html"
}
```
