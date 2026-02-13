# html/rendering/non-replaced-elements/flow-content-0/slot-element-default-display.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/flow-content-0/slot-element-default-display.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>CSS Test (Display): <slot> elements default display should be contents</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#flow-content-3">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>

<script>
    test(() => {
        let host = document.body.appendChild(document.createElement("div"));
        let slot = host.attachShadow({ mode: "open" }).appendChild(document.createElement("slot"));
        let cs = getComputedStyle(slot);
        assert_equals(cs.getPropertyValue("display"), "contents", "slot default display is not contents");
        document.body.removeChild(host);
    }, `slot element with default display should be contents`);

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
  "source_name": "html/rendering/non-replaced-elements/flow-content-0/slot-element-default-display.html"
}
```
