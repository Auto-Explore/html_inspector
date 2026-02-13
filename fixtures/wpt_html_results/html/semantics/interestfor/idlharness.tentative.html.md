# html/semantics/interestfor/idlharness.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/idlharness.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org" />
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/WebIDLParser.js"></script>
<script src="/resources/idlharness.js"></script>

<script>
  idl_test(["interest-invokers.tentative"],
  ["html","dom","SVG"], (idl_array) => {
    idl_array.add_objects({
      HTMLAnchorElement: ['document.createElement("a")'],
      HTMLAreaElement: ['document.createElement("area")'],
      HTMLButtonElement: ['document.createElement("button")'],
      SVGAElement: ['document.createElementNS("http://www.w3.org/2000/svg", "a")'],
      InterestEvent: ['new InterestEvent("interest")']
    });
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interestfor/idlharness.tentative.html"
}
```
